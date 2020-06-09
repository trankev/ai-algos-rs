import argparse
import functools
import os

import tensorflow as tf


def build_network(
    state_size,
    hidden_layer_sizes,
    plies_count,
    add_policy_output,
    add_value_output,
    learning_rate,
    policy_loss_weight,
    reg_loss_weight,
    value_loss_weight,
):
    layer = tf.compat.v1.placeholder(
        shape=[None, state_size],
        dtype=tf.float32,
        name="state_in",
    )
    if hidden_layer_sizes:
        layer = build_hidden_layers(layer, hidden_layer_sizes)
    losses = []
    policy_loss = build_policy_layer(
        input_layer=layer,
        plies_count=plies_count,
        loss_weight=policy_loss_weight,
    )
    if add_policy_output:
        losses.append(policy_loss)
    value_loss = build_value_layer(input_layer=layer, loss_weight=value_loss_weight)
    if add_value_output:
        losses.append(value_loss)
    setup_training(losses=losses, reg_loss_weight=reg_loss_weight, learning_rate=learning_rate)
    setup_saving()
    return tf.compat.v1.Session().graph_def


def build_hidden_layers(input_layer, hidden_layer_sizes, prefix="hidden_layer"):
    layer = input_layer
    for index, layer_size in enumerate(hidden_layer_sizes):
        layer = tf.layers.dense(
            layer,
            layer_size,
            activation=tf.nn.relu,
            kernel_initializer=tf.contrib.layers.variance_scaling_initializer(),
            kernel_regularizer=tf.contrib.layers.l1_l2_regularizer(),
            name="{}_{}".format(prefix, index),
        )
    return layer


def build_policy_layer(input_layer, plies_count, loss_weight):
    with tf.variable_scope("value", reuse=tf.AUTO_REUSE):
        policy_layer = tf.layers.dense(
            input_layer,
            plies_count,
            activation=tf.nn.softmax,
            kernel_initializer=tf.contrib.layers.variance_scaling_initializer(
            ),
            kernel_regularizer=tf.contrib.layers.l1_l2_regularizer(),
        )
    allowed_plies = tf.compat.v1.placeholder(
        shape=[None, plies_count],
        dtype=tf.float32,
        name="allowed_plies_in",
    )
    allowed_outputs = policy_layer * allowed_plies
    tf.math.divide(
        allowed_outputs,
        tf.math.reduce_sum(allowed_outputs),
        name="probabilities_out",
    )
    tf.argmax(allowed_outputs, 1, name="argmax_action_out")
    tf.random.categorical(tf.log(allowed_outputs), 1, name="stochastic_action_out")

    rewards_in = tf.compat.v1.placeholder(shape=[None], dtype=tf.float32, name="rewards_in")
    actions_in = tf.compat.v1.placeholder(shape=[None], dtype=tf.int32, name="actions_in")

    indices = tf.add(
        tf.range(0, tf.shape(allowed_outputs)[0]) * tf.shape(allowed_outputs)[1],
        actions_in,
        name="indices",
    )
    responsible_outputs = tf.gather(
        tf.reshape(policy_layer, [-1]),
        indices,
        name="responsible_outputs",
    )

    policy_loss = tf.multiply(
        -tf.math.reduce_mean(tf.math.log(responsible_outputs + 1e-9) * rewards_in),
        loss_weight,
        name="policy_loss_out",
    )
    return policy_loss


def build_value_layer(input_layer, loss_weight):
    value_layer = tf.layers.dense(
        input_layer,
        1,
        activation=tf.math.tanh,
        kernel_initializer=tf.contrib.layers.variance_scaling_initializer(),
        kernel_regularizer=tf.contrib.layers.l1_l2_regularizer(),
        name="qvalue_out",
    )
    tf.argmax(value_layer, 1, name="argmax_value_out")
    expected_value_in = tf.compat.v1.placeholder(
        shape=[None, 1],
        dtype=tf.float32,
        name="qvalues_in",
    )
    value_loss = tf.multiply(
        loss_weight,
        tf.compat.v1.losses.mean_squared_error(
            predictions=value_layer,
            labels=expected_value_in,
        ),
        name="value_loss_out",
    )
    return value_loss


def setup_training(losses, reg_loss_weight, learning_rate):
    reg_loss = tf.multiply(
        reg_loss_weight,
        tf.identity(tf.get_collection(tf.GraphKeys.REGULARIZATION_LOSSES)),
        name="reg_losses_out",
    )
    other_losses = functools.reduce(lambda x, y: tf.add(x, y), losses)

    total_loss = tf.add(reg_loss, other_losses, name="total_loss_out")
    optimizer = tf.train.AdamOptimizer(learning_rate=learning_rate)
    optimizer.minimize(total_loss, name="update_batch_op")


def setup_saving():
    tf.compat.v1.variables_initializer(
        tf.compat.v1.global_variables(),
        name="init_op",
    )
    tf.compat.v1.train.Saver(tf.compat.v1.global_variables())


def run(args):
    graph_def = build_network(
        state_size=args.state_size,
        hidden_layer_sizes=args.hidden_layer_sizes,
        plies_count=args.plies_count,
        add_value_output=args.add_value_output,
        add_policy_output=args.add_policy_output,
        value_loss_weight=args.value_loss_weight,
        policy_loss_weight=args.policy_loss_weight,
        reg_loss_weight=args.reg_loss_weight,
        learning_rate=args.learning_rate,
    )
    output_folder = os.path.dirname(args.output)
    output_file = os.path.basename(args.output)
    tf.compat.v1.train.write_graph(
        graph_def,
        output_folder,
        output_file,
        as_text=False,
    )


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-s", "--state-size", type=int)
    parser.add_argument("-H", "--hidden", dest="hidden_layer_sizes", nargs="+", type=int)
    parser.add_argument("-a", "--plies", dest="plies_count", type=int)
    parser.add_argument("-p", "--add-policy-output", action="store_true")
    parser.add_argument("-v", "--add-value-output", action="store_true")
    parser.add_argument("-l", "--learning-rate", default=0.001, type=float)
    parser.add_argument("-V", "--value-loss-weight", default=0.1, type=float)
    parser.add_argument("-P", "--policy-loss-weight", default=1.0, type=float)
    parser.add_argument("-R", "--reg-loss-weight", default=1.0e-5, type=float)
    parser.add_argument("-o", "--output", default="./data/network/model.pb")
    args = parser.parse_args()
    run(args)


if __name__ == "__main__":
    main()

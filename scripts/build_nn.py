import argparse
import os

import tensorflow as tf


def build_network(state_size, hidden_layer_sizes, action_count, learning_rate, beta_loss):
    layer = tf.compat.v1.placeholder(
        shape=[None, state_size],
        dtype=tf.float32,
        name="state_in",
    )
    allowed_plies = tf.compat.v1.placeholder(
        shape=[None, action_count],
        dtype=tf.float32,
        name="allowed_plies_in",
    )
    for index, layer_size in enumerate(hidden_layer_sizes):
        layer = tf.layers.dense(
            layer,
            layer_size,
            activation=tf.nn.relu,
            kernel_initializer=tf.contrib.layers.variance_scaling_initializer(),
            kernel_regularizer=tf.contrib.layers.l1_l2_regularizer(),
            name="hidden_layer_{}".format(index),
        )
    output_layer = tf.layers.dense(
        layer,
        action_count,
        activation=tf.nn.softmax,
        kernel_initializer=tf.contrib.layers.variance_scaling_initializer(),
        kernel_regularizer=tf.contrib.layers.l1_l2_regularizer(),
        name="output_layer",
    )
    allowed_outputs = output_layer * allowed_plies
    tf.div(
        allowed_outputs,
        tf.math.reduce_sum(allowed_outputs),
        name="probabilities_out",
    )
    tf.argmax(allowed_outputs, 1, name="argmax_action_out")
    tf.random.categorical(tf.log(allowed_outputs), 1, name="stochastic_action_out")

    reward_holder = tf.compat.v1.placeholder(shape=[None], dtype=tf.float32, name="rewards_in")
    action_holder = tf.compat.v1.placeholder(shape=[None], dtype=tf.int32, name="actions_in")

    indices = tf.add(
        tf.range(0, tf.shape(allowed_outputs)[0]) * tf.shape(allowed_outputs)[1],
        action_holder,
        name="indices",
    )
    responsible_outputs = tf.gather(
        tf.reshape(output_layer, [-1]),
        indices,
        name="responsible_outputs",
    )

    loss = tf.negative(
        tf.math.reduce_mean(
            tf.math.log(responsible_outputs + 1e-9) * reward_holder),
        name="policy_loss_out",
    )
    reg_loss = tf.multiply(
        beta_loss,
        tf.identity(tf.get_collection(tf.GraphKeys.REGULARIZATION_LOSSES)),
        name="reg_losses_out",
    )

    total_loss = tf.add(loss, reg_loss, name="total_loss_out")
    optimizer = tf.train.AdamOptimizer(learning_rate=learning_rate)
    optimizer.minimize(total_loss, name="update_batch_op")

    tf.compat.v1.variables_initializer(tf.compat.v1.global_variables(),
                                       name="init_op")
    tf.compat.v1.train.Saver(tf.compat.v1.global_variables())

    return tf.compat.v1.Session().graph_def


def run(args):
    graph_def = build_network(
        state_size=args.state_size,
        hidden_layer_sizes=args.hidden_layer_sizes,
        action_count=args.action_count,
        beta_loss=args.beta_loss,
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
    parser.add_argument("state_size", type=int)
    parser.add_argument("hidden_layer_sizes", nargs="+", type=int)
    parser.add_argument("action_count", type=int)
    parser.add_argument("-l", "--learning-rate", default=0.001, type=float)
    parser.add_argument("-b", "--beta-loss", default=1.0e-5, type=float)
    parser.add_argument("-o", "--output", default="./data/network/model.pb")
    args = parser.parse_args()
    run(args)


if __name__ == "__main__":
    main()

import argparse
import functools
import json
import os

import tensorflow as tf


def run(settings, output_file):
    graph_def = build_network(settings)
    output_folder = os.path.dirname(output_file)
    output_file = os.path.basename(output_file)
    tf.compat.v1.train.write_graph(
        graph_def,
        output_folder,
        output_file,
        as_text=False,
    )


def build_network(settings):
    state_shape = [None]
    state_shape.extend(settings["state_dimensions"])
    state_in = tf.placeholder(tf.float32, shape=state_shape, name="state_in")
    is_training = tf.placeholder(tf.bool, name="is_training_in")
    dropout = tf.placeholder(tf.float32)
    flat_conv = build_conv_layers(state_in, settings, is_training)
    s_fc = build_dense_layers(flat_conv, settings, is_training, dropout)
    policy_loss = build_policy_layer(s_fc, settings)
    value_loss = build_value_layer(s_fc, settings)
    setup_training([policy_loss, value_loss], settings)
    setup_saving()
    return tf.compat.v1.Session().graph_def


def build_conv_layers(layer_in, settings, is_training):
    image_shape = [-1]
    image_shape.extend(settings["state_dimensions"])
    image_shape.append(1)
    h_conv = tf.reshape(layer_in, shape=image_shape)
    conv_settings = settings["convolution"]
    reduce_count = 0
    for layer_settings in conv_settings["layers"]:
        h_conv = tf.nn.relu(
            tf.layers.batch_normalization(
                tf.layers.conv2d(
                    h_conv,
                    conv_settings["channel_count"],
                    kernel_size=conv_settings["kernel_size"],
                    padding="same" if layer_settings["same_padding"] else "valid",
                ),
                axis=3,
                training=is_training,
            )
        )
        if not layer_settings["same_padding"]:
            reduce_count += 2
    output_size = functools.reduce(
        lambda x, y: x * (y - reduce_count),
        settings["state_dimensions"],
        conv_settings["channel_count"],
    )
    flat_conv = tf.reshape(h_conv, [-1, output_size])
    return flat_conv


def build_dense_layers(layer_in, settings, is_training, dropout):
    s_fc = layer_in
    for layer_settings in settings["dense"]["layers"]:
        s_fc = tf.layers.dropout(
            tf.nn.relu(
                tf.layers.batch_normalization(
                    tf.layers.dense(
                        s_fc,
                        layer_settings["size"],
                        use_bias=layer_settings["use_bias"],
                    ),
                    axis=1,
                    training=is_training,
                )
            ),
            rate=dropout,
        )
    return s_fc


def build_policy_layer(layer_in, settings):
    pis = tf.layers.dense(layer_in, settings["action_count"])
    tf.nn.softmax(pis, name="probs_out")
    target_pis = tf.placeholder(
        tf.float32,
        shape=[None, settings["action_count"]],
        name="target_pis_in",
    )
    loss_pi = tf.losses.softmax_cross_entropy(target_pis, pis)
    return loss_pi


def build_value_layer(layer_in, setttings):
    value = tf.nn.tanh(
        tf.layers.dense(layer_in, 1),
        name="value_out"
    )
    target_value = tf.placeholder(tf.float32, shape=[None])
    loss_value = tf.losses.mean_squared_error(target_value, tf.reshape(value, shape=[-1]))
    return loss_value


def setup_training(losses, settings):
    cumulated_loss = functools.reduce(lambda x, y: tf.add(x, y), losses[:-1])
    total_loss = tf.add(cumulated_loss, losses[-1], name="total_loss_out")
    update_ops = tf.get_collection(tf.GraphKeys.UPDATE_OPS)
    with tf.control_dependencies(update_ops):
        optimizer = tf.train.AdamOptimizer(settings["learning_rate"])
        optimizer.minimize(total_loss, name="total_loss_out")


def setup_saving():
    tf.compat.v1.variables_initializer(
        tf.compat.v1.global_variables(),
        name="init_op",
    )
    tf.compat.v1.train.Saver(tf.compat.v1.global_variables())


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("project_folder")
    args = parser.parse_args()
    settings_file = os.path.join(args.project_folder, "settings.json")
    with open(settings_file) as fd:
        settings = json.load(fd)
    output_file = os.path.join(args.project_folder, "model.pb")
    run(settings, output_file)


if __name__ == "__main__":
    main()

""" Neural Network.

A 2-Hidden Layers Fully Connected Neural Network (a.k.a Multilayer Perceptron)
implementation with TensorFlow. This example is using the MNIST database
of handwritten digits (http://yann.lecun.com/exdb/mnist/).

This example is using TensorFlow layers, see 'neural_network_raw' example for
a raw implementation with variables.

Links:
    [MNIST Dataset](http://yann.lecun.com/exdb/mnist/).

Author: Aymeric Damien
Project: https://github.com/aymericdamien/TensorFlow-Examples/
"""

from __future__ import print_function

# Import MNIST data
#from tensorflow.examples.tutorials.mnist import input_data
#mnist = input_data.read_data_sets("/tmp/data/", one_hot=False)

import tensorflow as tf
import numpy

import subprocess

numpy.set_printoptions(threshold=numpy.inf)

def print_model(f, model):
    for name in model.get_variable_names():
        na = model.get_variable_value(name)
        if na.ndim == 2:
            na = numpy.transpose(na)
        l = list(na.flat)
        f.write(str(len(l)) + "\n")
        for i in range(len(l)):
            item = l[i]
            f.write(str(item) + " ")
        f.write("\n")

def print_model_rust(f, model):
    for name in model.get_variable_names():
        na = model.get_variable_value(name)
        if na.ndim == 2:
            na = numpy.transpose(na)
        l = list(na.flat)
        f.write("let " + name.replace("/", "_") + ": Vec<f32> = vec![")
        for i in range(len(l) - 1):
            item = l[i]
            f.write(str("%.3f" % item) + ", ")
        f.write(str(l[len(l) - 1]))
        f.write("];")
        f.write("\n")

# Parameters
learning_rate = 0.1
num_steps = 100
batch_size = 100
display_step = 100

# Network Parameters
n_hidden_1 = 57 # 1st layer number of neurons
n_hidden_2 = 57 # 2nd layer number of neurons
num_input = 81 # Board size is 9x9, so 81, 0 for no player, -1 for O and 1 for X
num_output = 82 # One for each possible action, and one for the eval (-1 if to_move loses, 1 if to_move wins)

training_inputs = []
training_labels = []

training_file = open("random_train.txt", "r")

training_lines = training_file.readlines()

for i in range(500):
  training_inputs.append(numpy.array([float(x) for x in filter(None, training_lines[i*2].split())]))  
  training_labels.append(numpy.array([float(x) for x in filter(None, training_lines[i*2 + 1].split())]))

training_inputs = numpy.array(training_inputs)
training_labels = numpy.array(training_labels)

test_inputs = []
test_labels = []


test_file = open("random_test.txt", "r")

test_lines = test_file.readlines()

for i in range(500):
  test_inputs.append(numpy.array([float(x) for x in filter(None, test_lines[i*2].split())]))  
  test_labels.append(numpy.array([float(x) for x in filter(None, test_lines[i*2 + 1].split())]))

test_inputs = numpy.array(test_inputs)
test_labels = numpy.array(test_labels)

def examples_from_str(s):
    inputs = []
    labels = []
    lines = s.decode('utf-8').split("\n")
    for i in range(len(lines)//2):
        inputs.append(numpy.array([float(x) for x in filter(None, lines[i*2].split())]))  
        labels.append(numpy.array([float(x) for x in filter(None, lines[i*2 + 1].split())]))
    return inputs, labels

# Define the neural network
def neural_net(x_dict):
    # TF Estimator input is a dict, in case of multiple inputs
    x = x_dict['board_state']
    # Hidden fully connected layer with 256 neurons
    layer_1 = tf.layers.dense(x, n_hidden_1)
    # Hidden fully connected layer with 256 neurons
    layer_2 = tf.layers.dense(layer_1, n_hidden_2)
    # Output fully connected layer with a neuron for output
    out_layer = tf.layers.dense(layer_2, num_output)
    return out_layer


# Define the model function (following TF Estimator Template)
def model_fn(features, labels, mode):
    # Build the neural network
    logits = neural_net(features)

    # If prediction mode, early return
    if mode == tf.estimator.ModeKeys.PREDICT:
        return tf.estimator.EstimatorSpec(mode, predictions=logits)

    v = tf.clip_by_value(tf.slice(logits, [0, 81], [batch_size,1]), -1.0, 1.0)
    v_label = tf.slice(labels, [0, 81], [batch_size, 1])

    moves = tf.clip_by_value(tf.slice(logits, [0, 0], [batch_size, 81]), 0.000001, 1.0)
    moves_label = tf.clip_by_value(tf.slice(labels, [0, 0], [batch_size, 81]), 0.0000001, 1.0) 

    # Define loss and optimizer
    # TODO
    #print(logits[0])
    #loss_op = tf.reduce_mean(tf.squared_difference(v, v_label)) - tf.reduce_sum(tf.multiply(tf.log(moves), moves_label), [0, 1], keep_dims=True)
    #print(loss_op)
    print(v)
    loss_op = tf.reduce_mean(tf.squared_difference(moves, moves_label)) + 81 * tf.reduce_mean(tf.squared_difference(v, v_label))

    optimizer = tf.train.GradientDescentOptimizer(learning_rate=learning_rate)

    train_op = optimizer.minimize(loss_op,
                                  global_step=tf.train.get_global_step())

    # Evaluate the accuracy of theodel
    acc_op = tf.metrics.mean_absolute_error(labels=labels, predictions=logits)

    # TF Estimators requires to return a EstimatorSpec, that specify
    # the different ops for training, evaluating, ...
    estim_specs = tf.estimator.EstimatorSpec(
        mode=mode,
        predictions=logits,
        loss=loss_op,
        train_op=train_op,
        eval_metric_ops={'mean_absolute_error': acc_op})

    return estim_specs

def self_play(model):
    model_name = "model_self_play"
    f = open(model_name + ".txt", "w+")
    print_model(f, model)
    f.close()
    f_rust = open(model_name + "_rust.txt", "w+")
    print_model_rust(f_rust, model)
    process = subprocess.Popen(("./target/release/genexamples " + model_name + ".txt").split(), stdout=subprocess.PIPE)
    output, error = process.communicate()
    return examples_from_str(output)
    

def pit(model_1, model_2):
    f1 = open("model_1.txt", "w+")
    print_model(f1, model_1)
    f1.close()
    f2 = open("model_2.txt", "w+")
    print_model(f2, model_2)
    f2.close()
    process = subprocess.Popen("./target/release/pit model_1.txt model_2.txt".split(), stdout=subprocess.PIPE)
    output, error = process.communicate()
    output = output.split()
    model_1_score = int(output[0])
    model_2_score = int(output[1])
    return model_2_score - model_1_score


def train(model, inputs, labels):
    input_fn = tf.estimator.inputs.numpy_input_fn(
        x={'board_state': inputs}, y=labels,
        batch_size=batch_size, num_epochs=None, shuffle=True)
    # Train the Model (on random data)
    model.train(input_fn, steps=num_steps)

    

# Build the Estimator
model = tf.estimator.Estimator(model_fn, model_dir="./temp")

# Define the input function for training
input_fn = tf.estimator.inputs.numpy_input_fn(
    x={'board_state': training_inputs}, y=training_labels,
    batch_size=batch_size, num_epochs=None, shuffle=True)
# Train the Model (on random data)
model.train(input_fn, steps=num_steps)

f_rust = open("best_model_rust.txt", "w+")
print_model_rust(f_rust, model)
f_rust.close()

iteration = 1

best_dir = "./temp"

inputs = []
labels = []

while(True):
    _inputs, _labels = self_play(model)
    inputs += _inputs
    labels += _labels
    new_model = tf.estimator.Estimator(model_fn, model_dir="./temp" + str(iteration), warm_start_from=best_dir)
    
    train(new_model, numpy.array(inputs), numpy.array(labels))

    pit_result = pit(model, new_model)

    if pit_result > 5:
        model = new_model
        best_dir = "./temp" + str(iteration) 
        #inputs = []
        #labels = []

    print(pit_result)
    
    f_rust = open("best_model_rust.txt", "w+")
    print_model_rust(f_rust, model)
    f_rust.close()
    iteration += 1




"""
# Define the input function for training
input_fn = tf.estimator.inputs.numpy_input_fn(
    x={'board_state': training_inputs}, y=training_labels,
    batch_size=batch_size, num_epochs=None, shuffle=True)
# Train the Model
model.train(input_fn, steps=num_steps)

# Evaluate the Model
# Define the input function for evaluating
input_fn = tf.estimator.inputs.numpy_input_fn(
    x={'board_state': test_inputs}, y=test_labels,
    batch_size=batch_size, shuffle=False)
# Use the Estimator 'evaluate' method
e = model.evaluate(input_fn)

input_fn = tf.estimator.inputs.numpy_input_fn(
    x={'board_state': numpy.array([test_inputs[0]])}, y=None,
    batch_size=batch_size, shuffle=False)
print(test_inputs[0])

print(model.get_variable_value("dense/kernel").shape)
#print(model.get_variable_value("dense/kernel"))

t_i = list(test_inputs[0].flat)
f_rust.write("let test_input: Vec<f32> = vec![")
for i in range(len(t_i) - 1):
    item = t_i[i]
    f_rust.write(str(item) + ", ")
f_rust.write(str(t_i[len(t_i) - 1]))
f_rust.write("];")
f_rust.write("\n")


t_i = next(model.predict(input_fn))
f_rust.write("let test_output: Vec<f32> = vec![")
for i in range(len(t_i) - 1):
    item = t_i[i]
    f_rust.write(str(item) + ", ")
f_rust.write(str(t_i[len(t_i) - 1]))
f_rust.write("];")
f_rust.write("\n")
""" 


#print("Testing Accuracy:", e['mean_absolute_error'])

#f_rust.close()

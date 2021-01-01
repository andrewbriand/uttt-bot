import numpy as np
import math
import torch
import torch.nn as nn
import random
from encoder import *

torch.autograd.set_detect_anomaly(True)


relu = torch.nn.ReLU()

#enc = Encoder(60, 1, 1)
#enc.load_state_dict(torch.load("encoder.torch"))

class Net(nn.Module):
    def __init__(self, h, k, p):
        super(Net, self).__init__()

        self.f0 = nn.Linear(18, k)
        self.f2 = nn.Linear(k, k)
        self.f3 = nn.Linear(k, k//2)
        self.f1 = nn.Linear(k//2, p)

        #self.fc1 = nn.Linear(90, h)
        self.fc3 = nn.Linear(10*p, h)
        self.fc4 = nn.Linear(h, h//2)
        self.fc5 = nn.Linear(h//2, h//4)
        #self.fc4 = nn.Linear(h//2, 9)
        self.fc2 = nn.Linear(h//4, 1)
        #self.d = nn.Dropout(p=0.1)

    def forward_square(self, x_in):
        x = relu(self.f0(x_in))
        x = relu(self.f2(x))
        x = relu(self.f3(x))
        x = relu(self.f1(x))
        return x

    def forward(self, x_in):
        #x = enc.encode(x_in)
        #x = self.d(x)
        #x = torch.cat((x, self.forward_square(x_in[:, 0:9])), 1)
        x = self.forward_square(torch.cat((x_in[:, 0:9], x_in[:, 90:99]), 1))
        #print(x.shape)
        for i in range(1, 10):
            x = torch.cat((x, self.forward_square(torch.cat((x_in[:, (i*9):((i*9)+9)], x_in[:, (i*9 + 90):((i*9 + 90) + 9)]), 1))), 1)
            #print(x.shape)
        #x = relu(self.fc1(x))
        x = relu(self.fc3(x))
        x = relu(self.fc4(x))
        x = relu(self.fc5(x))
        #x = self.forward_square(x)
        x = relu(self.fc2(x))
        return x


training_inputs = []
training_labels = []

training_file = open("gen_eval_train_5000ms_1231.txt", "r")

training_lines = training_file.readlines()

random.shuffle(training_lines)

for l in training_lines:
  nums = [float(x) for x in filter(None, l.split())]
  training_inputs.append(np.array(nums[0:90]))
  training_labels.append(np.array(nums[90:91]))

training_inputs_ = np.array(training_inputs)
training_labels_ = np.array(training_labels)

neg_training_inputs = -training_inputs_
neg_training_labels = 10.0 - training_labels_

training_inputs_ = np.r_[training_inputs_, neg_training_inputs]
training_labels_ = np.r_[training_labels_, neg_training_labels]

train_size = int(0.9 * len(training_inputs))

training_inputs = training_inputs_[0:train_size]
valid_inputs = training_inputs_[train_size:]

training_labels = training_labels_[0:train_size]
valid_labels = training_labels_[train_size:]

test_inputs = []
test_labels = []

test_file = open("gen_eval_test_5000ms.txt", "r")

test_lines = test_file.readlines()

for l in test_lines:
  nums = [float(x) for x in filter(None, l.split())]
  test_inputs.append(np.array(nums[0:90]))
  test_labels.append(np.array(nums[90:91]))

test_inputs = np.array(test_inputs)
test_labels = np.array(test_labels)

X_train = (torch.from_numpy(training_inputs).float())
labels_train = torch.from_numpy(training_labels).float() / 10.0
X_test = (torch.from_numpy(test_inputs).float())
labels_test = torch.from_numpy(test_labels).float() / 10.0
X_valid = (torch.from_numpy(valid_inputs).float())
labels_valid = torch.from_numpy(valid_labels).float() / 10.0

X_train = torch.cat((X_train == -1.0, X_train == 1.0), 1).float()
X_test = torch.cat((X_test == -1.0, X_test == 1.0), 1).float()
X_valid = torch.cat((X_valid == -1.0, X_valid == 1.0), 1).float()

relu = torch.nn.ReLU()
            
import torch.optim as optim

def WeightLoss(x, y):
    #return nn.MSELoss()(torch.pow((x - 0.5), 2), torch.pow((y - 0.5), 2))
    return torch.mean(torch.pow(x - y, 2) * torch.abs(y - 0.5))

for h in [120]:
    for k in [8]:
        for p in [30]:
            for reg in [0.0000]:
                for criterion in [nn.MSELoss()]:
                    lr_ = 0.0005
                    min_valid_mdist = 10000
                    #net = Net(10, 10, 10)
                    print(str(h) + " " + str(k) + " " + str(p))
                    print(reg)
                    print(criterion)
                    net = Net(h, k, p)
                    
                    
                    #criterion = nn.MSELoss()
                    #criterion = WeightLoss
                    
                    optimizer = optim.Adam(net.parameters(), lr=lr_, weight_decay=reg)


                    last_loss = -1000.0
                    
                    num_epochs = 1600
                    for epoch in range(num_epochs):
                        optimizer.zero_grad()
                        outputs = net(X_train)
                        loss = criterion(outputs, labels_train)
                        loss.backward()
                        optimizer.step()

                        if epoch == 100:
                            criterion = nn.MSELoss()

                        if loss == last_loss:
                            epoch = 0
                            net = Net(h, k, p)
                            optimizer = optim.Adam(net.parameters(), lr=lr_, weight_decay=reg)

                        last_loss = loss
                        #train_mse = nn.MSELoss()(outputs, labels_train)
                    
                        valid_outputs = net(X_valid)
                        #valid_loss = criterion(valid_outputs, labels_valid)
                        valid_mdist = torch.mean(torch.abs(valid_outputs - labels_valid))
                        train_mdist = torch.mean(torch.abs(outputs - labels_train))
                        #extr_indices = labels_valid - .5 > .1
                        #close_indices = labels_valid - .5 < .1
                        #valid_extr_mdist = torch.mean(torch.abs(valid_outputs[extr_indices] - labels_valid[extr_indices]))
                        #valid_close_mdist = torch.mean(torch.abs(valid_outputs[close_indices] - labels_valid[close_indices]))
                        if valid_mdist < min_valid_mdist:
                            min_valid_mdist = valid_mdist
                        if epoch == num_epochs - 1:
                            #print(valid_outputs)
                            #print(labels_valid)
                            print("min valid mdist: " + str(min_valid_mdist.item()))
                    
                        print("epoch: " + str(epoch) + "\ttrain loss: " + str(round(loss.item(), 4)) + "\ttrain mdist: " + str(round(train_mdist.item(), 4)) + "\tvalid mdist: " + str(round(valid_mdist.item(), 4)))
                        #print("valid extr mdist: " + str(valid_extr_mdist.item()))
                        #print("valid close mdist: " + str(valid_close_mdist.item()))
                        #print("")
                    
                    #print(torch.reshape(net.f0.weight, (-1, 3, 3)))



"""with torch.no_grad():
  outputs = net(X_test)
  loss = criterion(outputs, labels_test)
  #print("Test loss: " + str(loss))"""


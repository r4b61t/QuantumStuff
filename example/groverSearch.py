import numpy as np
import quantum_stuff as qs

#search for bit 00

state = qs.Register(3)

coeff = 1/np.sqrt(2)
H = qs.QGate([coeff],[coeff],[coeff],[-coeff])
S = qs.QGate([1],[0],[0],[0,1])
X = qs.QGate([0],[1],[1],[0])

state.apply_gate(H,[0,1],[])
state.apply_gate(S,[0,1],[])
state.apply_gate(H,[1],[])
state.apply_gate(X,[1],[0])
state.apply_gate(H,[1],[])
state.apply_gate(S,[0,1],[])
state.apply_gate(H,[0,1],[])
state.apply_gate(X,[0,1],[])
state.apply_gate(H,[1],[])
state.apply_gate(X,[1],[0])
state.apply_gate(H,[1],[])
state.apply_gate(X,[0,1],[])
state.apply_gate(H,[0,1],[])

print(state.probabilities())

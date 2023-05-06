import quantum_stuff as qs
import numpy as np
import time

coeff = 1/np.sqrt(2)

state = qs.Register(20)
Hadamard = qs.QGate([coeff],[coeff],[coeff],[-coeff])

a= time.time()
for i in range(20):
    state.apply_gate(Hadamard,[i],[])

print(state.measure())
b = time.time()
print(b-a)


from qiskit import QuantumCircuit,Aer,transpile

a= time.time()
circuit = QuantumCircuit(20)

for i in range(20):
    circuit.h(i)


# Create a Quantum Circuit
meas = QuantumCircuit(20,20)
meas.barrier(range(20))
# map the quantum measurement to the classical bits
meas.measure(range(20), range(20))

# The Qiskit circuit object supports composition using
# the compose method.
circuit.add_register(meas.cregs[0])
qc = circuit.compose(meas)
# Use Aer's qasm_simulator
backend_sim = Aer.get_backend('qasm_simulator')

# Execute the circuit on the qasm simulator.
# We've set the number of repeats of the circuit
# to be 1024, which is the default.
job_sim = backend_sim.run(transpile(qc, backend_sim), shots=1)

# Grab the results from the job.
result_sim = job_sim.result()
counts = result_sim.get_counts(qc)
print(counts)
b = time.time()
print(b-a)

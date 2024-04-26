# Sketches

In the [transfer-functions subfolder](./transfer-functions/) you can find LTspice simulations of some circuit parts that are emulated. And for some of these circuit parts it contains corresponding filter transfer functions created in python. This way you can check if the plots from the python code show the same frequency response as the LTspice plots. The printed filter coefficients can then be used as input for unit tests in the Rust implementations.

In the [fir-filter subfolder](./fir-filter/) you can find the code that generates the FIR filter tables for oversampling.

## Transfer functions

The LTspice simulations and python transfer functions can be found [here](./transfer-functions).

#### Running this python code

1. Go into the [transfer-functions folder](./transfer-functions)
2. Run `pip3 install -r requirements.txt`.
3. Run the file you want to get the filter coefficient for or to see the frequency response plots. `python3 ./<file name>`

## FIR filter

The FIR filter tables for oversampling are generated with [this python code](./fir-filter/fir.py).

#### Running this python code

1. Go to the [fir-filter directory](./fir-filter/).
2. Run `pip3 install -r requirements.txt`.
3. Change the oversampling variable to the oversampling factor you'd like.
4. Run the script `python3 ./fir.py`. The FIR filter table coefficients are then logged on your console.

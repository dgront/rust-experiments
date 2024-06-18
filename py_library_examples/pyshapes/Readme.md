For the very first time: create and activate virtual environment, install maturin:
``
python3.9 -m venv venv
source venv/bin/activate
pip3 install maturin
``


Any other time: have virtual environment activated and call:
``
maturin develop
``

to build the package. Then try if the ``py_examples/demo.py`` works.

The ``shapes`` library is also available as a rust module. To sompile an example, just call:

``
cargo build --release --examples
``

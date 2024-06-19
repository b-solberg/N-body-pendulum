### N Body Pendulum

---

This is a n-body Pbendulum simulation and animation using pure Rust. Several n body pendulum system have been solved in many other repos such as: 

- (http://travisrtaylor.com/pendulum-explainer/)
- (https://github.com/NaokiHori/Pendulum)

These are great and there are many others! Most simulations reduce the problem to a system with pendulum lengths of 1 meter and pendulum masses to 1kg. I wanted to generalize the n body pendulum system to arbitrary length and mass pendulums. In the RK4 folder there are two functions that determine coefficients for various terms in the matrices to be solved. This is the major difference to other implementations. 

As is, the simulation does default to lengths and masses to 1 meter or kilogram. This can easily be changed by modifying the vectors for those parameters. 

The animation is handled through Rust's Nannou crate. This opens a window, animates in real time and also provides a GUI to chnage the number of pendulums. You can find the executable in target\release\pendulum.exe. 

TODO: Build a function to save the animation into a gif or other video format.

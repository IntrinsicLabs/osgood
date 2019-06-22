Osgood is a platform for running JavaScript on the server. It aims to be
secure, fast, and simple.

## Secure

Osgood applications are configured by specifying a list of route patterns and
indicating which Osgood worker to invoke. Each Osgood worker has its own set of
policies and are isolated from other workers.

## Fast

Osgood typically runs faster than equivalent technologies. For example, on a
benchmark where JavaScript returns `Hello, world!`, Osgood runs about 20%
faster than Node.js.

## Simple

Running an Osgood application is as simple as downloading a static [osgood
binary](Installation) and passing in the path to an application file, and
writing a second Osgood Worker file to handle the application logic.

* * *

Check out the [install](Installation) page if you'd like to install Osgood.

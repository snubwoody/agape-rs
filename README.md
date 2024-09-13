# Rust graphics library

I want the ui to be composed of individual special purpose widgets. That means a row should be a row and an image should be an image only for example. As opposed to html where anything can have any properties.  

I want to write this api in a way that I will switch to webgpu, vulkano or a more feature rich api once
I get the basics up and running. If that's even a good idea

## Goals

- Make it easy to build good looking and functional UI
- Producing readable code that doesn't feel like a pain to write
- Code must be descriptive ie the reader must be able to tell the layout of a page without seeing the output

I need to start doing these in order now

## Todo

- Improve the layout system
  - Add a flex widget
- Add a size struct
- Improve the event system
- Add rounded corners
- Add borders

## Ideas

For an on-hover, we keep all the bounds of the widgets in a hash map, then each frame we check
if the mouse position is one of the widgets then we call the function.

We can call the functions that change things about the widgets modifier functions.


Okay we have a `kind-of` working library now, not very usefull but still it works. So I think now I 
can start planning things and stop wondering about different things randomly.

## Projects

## Issues

### Image size issue

When the window is smaller than the image, the image resizes to fit the window which shouldn't happen


## Layout system

Widgets should have intrinsic sizes, which means they have a size that they want to be. I.e:

- Fill the parent
- Fit the children
- Relative
- Flex 
- Fixed

This should allow us to define complex layouts, which not-so-complex code. The widget_tree starts at the root
widgets and goes down the tree depth first calculating the layouts. 



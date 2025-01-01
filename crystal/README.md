# Crystal

A layout library

## Usage

There are four types of layout

- `HorizontalLayout`: Arranges it's children horizontally
- `VerticalLayout`: Arranges it's children vertically
- `BlockLayout`: Has only one child
- `EmptyLayout`: Has no children

The layout system is based on intrinsic sizes, there are there kinds of intrinsic sizing

- `Flex`: A node wants to be as large as possible
- `Shrink`: A node wants to be as large as possible
- `Fixed`: A node wants to be as large as possible

## Error handling

Errors are non-blocking, an error occuring for one componenet usually doesn't mean so everything else should halt so each `Layout` keeps an error stack that can be fetched from the root `Layout`. This way trivial errors like `overflow` and `out-of-bounds` can still be reported while the rest of the system continues. This also however means that if a parent experienced an error then the children will be affected as well.

## Scrolling

Vertical and Horizontal layouts should have a scroll offset that added to the position.

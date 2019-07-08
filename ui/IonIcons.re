module Link = {
  [@bs.module] [@react.component]
  external make:
    (
      ~className: string=?,
      ~fontSize: string=?,
      ~color: string=?,
      ~onClick: 'event => unit=?
    ) =>
    React.element =
    "react-ionicons/lib/IosLink";
};

module Download = {
  [@bs.module] [@react.component]
  external make:
    (
      ~className: string=?,
      ~fontSize: string=?,
      ~color: string=?,
      ~onClick: 'event => unit=?
    ) =>
    React.element =
    "react-ionicons/lib/MdDownload";
};

module Undo = {
  [@bs.module] [@react.component]
  external make:
    (
      ~className: string=?,
      ~fontSize: string=?,
      ~color: string=?,
      ~onClick: 'event => unit=?
    ) =>
    React.element =
    "react-ionicons/lib/MdUndo";
};

module Redo = {
  [@bs.module] [@react.component]
  external make:
    (
      ~className: string=?,
      ~fontSize: string=?,
      ~color: string=?,
      ~onClick: 'event => unit=?
    ) =>
    React.element =
    "react-ionicons/lib/MdRedo";
};

module Camera = {
  [@bs.module] [@react.component]
  external make:
    (
      ~className: string=?,
      ~fontSize: string=?,
      ~color: string=?,
      ~onClick: 'event => unit=?
    ) =>
    React.element =
    "react-ionicons/lib/MdCamera";
};

module ReverseCamera = {
  [@bs.module] [@react.component]
  external make:
    (
      ~className: string=?,
      ~fontSize: string=?,
      ~color: string=?,
      ~onClick: 'event => unit=?
    ) =>
    React.element =
    "react-ionicons/lib/MdReverseCamera";
};

module Document = {
  [@bs.module] [@react.component]
  external make:
    (
      ~className: string=?,
      ~fontSize: string=?,
      ~color: string=?,
      ~onClick: 'event => unit=?
    ) =>
    React.element =
    "react-ionicons/lib/MdDocument";
};

module ArrowDown = {
  [@bs.module] [@react.component]
  external make:
    (
      ~className: string=?,
      ~fontSize: string=?,
      ~color: string=?,
      ~onClick: 'event => unit=?
    ) =>
    React.element =
    "react-ionicons/lib/MdArrowDropdown";
};

module ArrowRight = {
  [@bs.module] [@react.component]
  external make:
    (
      ~className: string=?,
      ~fontSize: string=?,
      ~color: string=?,
      ~onClick: 'event => unit=?
    ) =>
    React.element =
    "react-ionicons/lib/MdArrowDropright";
};

module Flashlight = {
  [@bs.module] [@react.component]
  external make:
    (
      ~className: string=?,
      ~fontSize: string=?,
      ~color: string=?,
      ~onClick: 'event => unit=?
    ) =>
    React.element =
    "react-ionicons/lib/MdFlash";
};
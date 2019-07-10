/*
 content="Tooltip"
 arrow={true}
 animation="scale"
 duration={0}
 delay={[300, 0]}
 */

// import Tippy from '@tippy.js/react'

module Base = {
  [@bs.module "@tippy.js/react"] [@react.component]
  external make:
    (
      ~content: string,
      ~arrow: bool=?,
      ~animation: string=?,
      ~enabled: bool=?,
      ~duration: int=?,
      ~children: React.element=?
    ) =>
    React.element =
    "default";
};

[@react.component]
let make = (~content, ~children: React.element) =>
  <Base animation="fade" arrow=true enabled=true content> children </Base>;

module El = {
  [@bs.module "@tippy.js/react"] [@react.component]
  external make:
    (
      ~content: React.element,
      ~arrow: bool=?,
      ~animation: string=?,
      ~duration: int=?,
      ~children: React.element=?
    ) =>
    React.element =
    "default";
};
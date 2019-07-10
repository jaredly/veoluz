/*
 content="Tooltip"
 arrow={true}
 animation="scale"
 duration={0}
 delay={[300, 0]}
 */

// import Tippy from '@tippy.js/react'

[@bs.module "@tippy.js/react"] [@react.component]
external make:
  (
    ~content: string,
    ~arrow: bool=?,
    ~animation: string=?,
    ~duration: int=?,
    ~children: React.element=?
  ) =>
  React.element =
  "default";

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
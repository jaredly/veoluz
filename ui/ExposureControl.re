open Lets;

[@bs.val] external parseInt: (string, int) => float = "";

let colorToRgb = color => {
  let r = Js.String.substrAtMost(~from=1, ~length=2, color);
  let g = Js.String.substrAtMost(~from=3, ~length=2, color);
  let b = Js.String.substrAtMost(~from=5, ~length=2, color);
  (parseInt(r, 16), parseInt(g, 16), parseInt(b, 16));
};

let toHex = n =>
  (n < 16 ? "0" : "") ++ Js.Int.toStringWithRadix(n, ~radix=16);

let rgbToColor = ((r, g, b)) => {
  "#"
  ++ toHex(int_of_float(r))
  ++ toHex(int_of_float(g))
  ++ toHex(int_of_float(b));
};

module Colorpickr = {
  type color = {. "color": string};
  [@bs.module] [@react.component]
  external make: (~color: string, ~onChange: color => unit) => React.element =
    "rc-color-picker";
};

let evtPos = evt => (
  evt->ReactEvent.Mouse.clientX,
  evt->ReactEvent.Mouse.clientY,
);

let useDraggable = (~onMove) => {
  let (pressed, setPressed) = Hooks.useState(None);
  let moveRef = React.useRef(onMove);
  moveRef->React.Ref.setCurrent(onMove);

  React.useEffect3(
    () =>
      switch (pressed) {
      | None => None
      | Some((x, y)) =>
        let onMove = moveRef->React.Ref.current;
        let mousemove = evt => {
          evt->ReactEvent.Mouse.preventDefault;
          onMove(evtPos(evt));
        };
        let mouseup = evt => {
          evt->ReactEvent.Mouse.preventDefault;
          Js.log("Mouseup, remove");
          setPressed(None);
        };
        Web.window->Web.addEventListener("mousemove", mousemove, true);
        Web.window->Web.addEventListener("mouseup", mouseup, true);
        Some(
          () => {
            Js.log("releasing");
            Web.window->Web.removeEventListener("mousemove", mousemove, true);
            Web.window->Web.removeEventListener("mouseup", mouseup, true);
          },
        );
      // None
      },
    (pressed, (), ()),
  );

  (
    pressed,
    evt => {
      evt->ReactEvent.Mouse.preventDefault;
      let pos = evtPos(evt);
      setPressed(Some(pos));
      // onPress(pos);
    },
  );
};

let handleStyle =
  Css.(
    style([
      width(px(10)),
      height(px(10)),
      boxSizing(`borderBox),
      marginLeft(px(-5)),
      borderRadius(`percent(20.0)),
      cursor(`grab),
      backgroundColor(hex("fff")),
      border(px(1), `solid, hex("000")),
    ])
  );

let handleWrapperStyle = Css.(style([position(`absolute)
,
bottom(px(0))
]));

[@react.component]
let make = (~config, ~limits as (min_v, max_v), ~update, ~wasm: Rust.wasm, ~width) => {
  let containerRef = React.useRef(Js.Nullable.null);

  let (_, onMin) =
    useDraggable(~onMove=((x, y)) => {
      let%Opt.Consume container = containerRef->React.Ref.current->Js.toOption;
      let box = Web.getBoundingClientRect(container);
      let x = float_of_int(x) -. box##left;
      let y = float_of_int(y) -. box##top;
      let xPercent = x /. box##width;
      let config = [%js.deep
        config["rendering"]["exposure"]["limits"].replace(Js.Null.return((xPercent, max_v)))
      ];
      update(config, false);
    });

  let (_, onMax) =
    useDraggable(~onMove=((x, y)) => {
      let%Opt.Consume container = containerRef->React.Ref.current->Js.toOption;
      let box = Web.getBoundingClientRect(container);
      let x = float_of_int(x) -. box##left;
      let y = float_of_int(y) -. box##top;
      let xPercent = x /. box##width;
      let config = [%js.deep
        config["rendering"]["exposure"]["limits"].replace(Js.Null.return((min_v, xPercent)))
      ];
      update(config, false);
    });

  <div
    ref={ReactDOMRe.Ref.domRef(containerRef)}
    className=Css.(
      style([position(`absolute), bottom(px(0)), left(px(0)),
      ])
    )
    style={ReactDOMRe.Style.make(
      ~width=Js.Int.toString(width) ++ "px",
      ~height="40px",
      (),
    )}>
    <div
      style={ReactDOMRe.Style.make(
        ~left=
          Js.Float.toString(
            float_of_int(width)
            *.
            max(0.0, min_v),
          )
          ++ "px",
        (),
      )}
      className=handleWrapperStyle>
      <div onMouseDown=onMin className=handleStyle />
    </div>
    <div
      style={ReactDOMRe.Style.make(
        ~left=
          Js.Float.toString(
            float_of_int(width)
            *.
            min(1.0, max_v),
          )
          ++ "px",
        (),
      )}
      className=handleWrapperStyle>
      <div onMouseDown=onMax className=handleStyle />
    </div>
  </div>;
};
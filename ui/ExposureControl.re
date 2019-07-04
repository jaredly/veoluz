open Lets;

[@bs.val]
external parseInt: (string, int) => float = "";

let colorToRgb = color => {
  let r = Js.String.substrAtMost(~from=1, ~length=2, color);
  let g = Js.String.substrAtMost(~from=3, ~length=2, color);
  let b = Js.String.substrAtMost(~from=5, ~length=2, color);
  (
    parseInt(r, 16),
    parseInt(g, 16),
    parseInt(b, 16),
  )
};

let toHex = n => (n < 16 ? "0" : "") ++ Js.Int.toStringWithRadix(n, ~radix=16);

let rgbToColor = ((r, g, b)) => {
  "#" ++
  toHex(int_of_float(r)) ++
  toHex(int_of_float(g)) ++
  toHex(int_of_float(b))
}

module Colorpickr = {
  type color = {. "color": string};
  [@bs.module]
  [@react.component]
  external make: (~color: string, ~onChange: (color) => unit) => React.element = "rc-color-picker";
}

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
          onMove(evtPos(evt))
        };
        let mouseup = evt => {
          evt->ReactEvent.Mouse.preventDefault;
          Js.log("Mouseup, remove")
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

let handleStyle = Css.(style([
            width(px(10)),
            height(px(10)),
            boxSizing(`borderBox),
            marginLeft(px(-5)),
            borderRadius(`percent(20.0)),
            cursor(`grab),
            backgroundColor(hex("000")),
            border(px(2), `solid, hex("fff"))
        ]));


[@react.component]
let make = (~config, ~update, ~wasm) => {

  let containerRef = React.useRef(Js.Nullable.null);

  let (_, onMin) =
    useDraggable(~onMove=((x, y)) => {
      let%Opt.Consume container =
        containerRef->React.Ref.current->Js.toOption;
      let box = Web.getBoundingClientRect(container);
      let x = float_of_int(x) -. box##left;
      let y = float_of_int(y) -. box##top;
      let xPercent = x /. box##width;
      let config = [%js.deep
        config["rendering"]["exposure"]["min"].replace(xPercent)
      ];
      update(config, false);
    });

  let (_, onMax) =
    useDraggable(~onMove=((x, y)) => {
      let%Opt.Consume container =
        containerRef->React.Ref.current->Js.toOption;
      let box = Web.getBoundingClientRect(container);
      let x = float_of_int(x) -. box##left;
      let y = float_of_int(y) -. box##top;
      let xPercent = x /. box##width;
      let config = [%js.deep
        config["rendering"]["exposure"]["max"].replace(xPercent)
      ];
      update(config, false);
    });

  <div
    ref={ReactDOMRe.Ref.domRef(containerRef)}
    onMouseOver={evt => {
      wasm##show_hist();
    }}
    onMouseOut={evt => {
      wasm##hide_hist();
    }}
    className=Css.(style([
      position(`absolute),
      bottom(px(0)),
      left(px(0)),
    ]))
    style={ReactDOMRe.Style.make(
      ~width=Js.Int.toString(config##rendering##width) ++ "px",
      // ~position="relative",
      ~height="40px",
      // ~backgroundColor="#afa",
      // ~outline="2px solid black",
      (),
    )}>
    <div
      style={ReactDOMRe.Style.make(
        ~left=
          Js.Float.toString(
            float_of_int(config##rendering##width)
            *.
            config##rendering##exposure##min,
          )
          ++ "px",
        (),
      )}
      className=Css.(
        style([
          position(`absolute),
        ])
      )
    >
      <div 
        onMouseDown=onMin
        className=handleStyle
      />
      {
        switch ([%js.deep config##rendering##coloration["Rgb"]]) {
          | None => React.string("not rgb")
          | Some(rgb) => {
            <div
              style={ReactDOMRe.Style.make(
                ~width="10px",
                ~marginLeft="-13px",
                ~marginTop="2px",
                ~height="30px",
                ()
              )}
            >
              <Colorpickr
                color={rgbToColor(rgb##background)}
                onChange={color => {
                  Js.log2("Color", color);
                  let config = [%js.deep
                    config["rendering"]["coloration"]["Rgb"].map(rgb =>
                      switch (rgb) {
                      | None => None
                      | Some(v) => Some(v["background"].replace(colorToRgb(color##color)))
                      }
                    )
                  ];
                  update(config, false);
                }}
              />
            </div>
          }
        }
      }
    </div>

    <div
      style={ReactDOMRe.Style.make(
        ~left=
          Js.Float.toString(
            float_of_int(config##rendering##width)
            *.
            config##rendering##exposure##max,
          )
          ++ "px",
        (),
      )}
      className=Css.(
        style([
          position(`absolute),
        ])
      )
    >
      <div 
        onMouseDown=onMax
        className=handleStyle
      />
      {
        switch ([%js.deep config##rendering##coloration["Rgb"]]) {
          | None => React.string("not rgb")
          | Some(rgb) => {
            <div
              style={ReactDOMRe.Style.make(
                ~width="10px",
                ~marginLeft="-13px",
                ~marginTop="2px",
                ~height="30px",
                ()
              )}
            >
              <Colorpickr
                color={rgbToColor(rgb##highlight)}
                onChange={color => {
                  Js.log2("Color", color);
                  let config = [%js.deep
                    config["rendering"]["coloration"]["Rgb"].map(rgb =>
                      switch (rgb) {
                      | None => None
                      | Some(v) => Some(v["highlight"].replace(colorToRgb(color##color)))
                      }
                    )
                  ];
                  update(config, false);
                }}
              />
            </div>
          }
        }
      }
    </div>
  </div>
}

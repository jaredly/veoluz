type config = {
  .
  "lights":
    array({
      .
      "brightness": float,
      "kind": {
        .
        "Point": {
          .
          "offset": float,
          "origin": (float, float),
          "t0": float,
          "t1": float,
        },
      },
    }),
  "walls":
    array({
      .
      "kind": {
        .
        "Line":
          option({
            .
            "a": (float, float),
            "b": (float, float),
          }),
        "Parabola":
          option({
            .
            "a": float,
            "left": float,
            "right": float,
            "transform": {
              .
              "rotation": (float, float),
              "translation": (float, float),
            },
          }),
        "Circle":
          option(({. "radius": float}, (float, float), float, float)),
        // TODO circle
      },
      "properties": {
        .
        "absorb": float,
        "reflect": float,
        "roughness": float,
        "refraction": float,
      },
      "hide": bool,
    }),
  "transform": {
    .
    "rotational_symmetry": int,
    "reflection": bool,
  },
  "rendering": {
    .
    "center": (float, float),
    "coloration": {
      .
      "HueRange":
        option({
          .
          "start": float,
          "end": float,
          "saturation": float,
          "lightness": float,
        }),
      "Rgb":
        option({
          .
          "background": (float, float, float),
          "highlight": (float, float, float),
        }),
    },
    "exposure": {
      .
      "curve": string,
      "min": float,
      "max": float,
    },
    "height": int,
    "width": int,
    "zoom": float,
  },
};

type wasm = {
  .
  "initial": [@bs.meth] (unit => config),
  "setup": [@bs.meth] ((config, config => unit) => unit),
  "run": [@bs.meth] (unit => unit),
  "save": [@bs.meth] (unit => config),
  "restore": [@bs.meth] (config => unit),
  "update": [@bs.meth] ((config, bool) => unit),
  "blank_config": [@bs.meth] (unit => config),
  "parse_url_config": [@bs.meth] (string => Js.nullable(config))
};

let wasm: Js.Promise.t(wasm) = [%bs.raw {|import('../pkg/zenphoton')|}];

let withModule = fn =>
  wasm
  |> Js.Promise.then_(wasm => {
       fn(wasm);
       Js.Promise.resolve();
     })
  |> ignore;
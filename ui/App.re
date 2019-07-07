open Lets;

// [%bs.raw {|require("@mapbox/react-colorpickr/dist/colorpickr.css")|}];

type location;
[@bs.val] external location: location = "";

[@bs.set] external setHash: (location, string) => unit = "hash";

// module Colorpickr = {
//   type color = {. "r": float, "g": float, "b": float};
//   [@bs.module "@mapbox/react-colorpickr"]
//   [@react.component]
//   external make: (~onChange: (color) => unit) => React.element = "default";
// }

[%bs.raw {|require('rc-color-picker/assets/index.css')|}];

open Types;

module ExposureFunction = {
  let btn = Css.(style([disabled([backgroundColor(Colors.accent)])]));
  [@react.component]
  let make = (~config, ~update, ~wasm) => {
    let (isOpen, setOpen) = Hooks.useState(false);


    let canvasRef = React.useRef(Js.Nullable.null);

    React.useEffect1(
      () => {
        if (isOpen) {
          switch (Js.Nullable.toOption(canvasRef->React.Ref.current)) {
            | None => ()
            | Some(canvas) =>
            Js.log2("Sending canvas", canvas);
            wasm##show_hist(canvas->Web.asCanvas);
          }
        } else {
          wasm##hide_hist();
        };
        None;
      },
      [|isOpen|],
    );

    <div className=Styles.column>
      // className=Styles.title
      <div onClick={_ => setOpen(!isOpen)}
      className=Styles.join([Styles.row, Css.(style([
        cursor(`pointer),
        alignItems(`center),
        padding(px(4)),
        hover([
          backgroundColor(Colors.button)
        ])
      ]))])
      >
        {isOpen ? <IonIcons.ArrowDown fontSize="14px" /> : <IonIcons.ArrowRight fontSize="14px" />}
        {Styles.spacer(4)}
        {React.string("Exposure")}
      </div>
      <div
        style={ReactDOMRe.Style.make(~display=isOpen ? "flex" : "none", ())}
        className=Styles.column
      >
        {Styles.spacer(8)}
        <div className=Css.(style([position(`relative), paddingBottom(px(12)), marginBottom(px(8))]))>
          <canvas width="200px" height="100px" ref={ReactDOMRe.Ref.domRef(canvasRef)} />
          <ExposureControl wasm config update width=200 />
        </div>
        <div className=Styles.row>
          {switch ([%js.deep config##rendering##coloration["Rgb"]]) {
           | None => React.string("not rgb")
           | Some(rgb) =>
             <div
              //  className="color-picker-wrapper"
               style={ReactDOMRe.Style.make(
                //  ~width="10px",
                //  ~marginLeft="-13px",
                //  ~marginTop="2px",
                //  ~height="30px",
                 (),
               )}>
               <ExposureControl.Colorpickr
                 color={ExposureControl.rgbToColor(rgb##background)}
                 onChange={color => {
                   Js.log2("Color", color);
                   let config = [%js.deep
                     config["rendering"]["coloration"]["Rgb"].map(rgb =>
                       switch (rgb) {
                       | None => None
                       | Some(v) =>
                         Some(
                           v["background"].replace(
                             ExposureControl.colorToRgb(color##color),
                           ),
                         )
                       }
                     )
                   ];
                   update(config, false);
                 }}
               />
             </div>
           }}
        {Styles.spacer(4)}
          <input
            type_="number"
            value={Js.Float.toString(config##rendering##exposure##min)}
            className=Css.(style([width(px(70))]))
            onChange={evt => {
              let v = evt->ReactEvent.Form.target##value->float_of_string;
              let config = [%js.deep
                config["rendering"]["exposure"]["min"].replace(v)
              ];
              update(config, false);
            }}
          />
        {Styles.spacer(8)}
          {switch ([%js.deep config##rendering##coloration["Rgb"]]) {
           | None => React.string("not rgb")
           | Some(rgb) =>
             <div
              //  className="color-picker-wrapper"
               style={ReactDOMRe.Style.make(
                //  ~width="10px",
                //  ~marginLeft="-13px",
                //  ~marginTop="2px",
                //  ~height="30px",
                 (),
               )}>
               <ExposureControl.Colorpickr
                 color={ExposureControl.rgbToColor(rgb##highlight)}
                 onChange={color => {
                   Js.log2("Color", color);
                   let config = [%js.deep
                     config["rendering"]["coloration"]["Rgb"].map(rgb =>
                       switch (rgb) {
                       | None => None
                       | Some(v) =>
                         Some(
                           v["highlight"].replace(
                             ExposureControl.colorToRgb(color##color),
                           ),
                         )
                       }
                     )
                   ];
                   update(config, false);
                 }}
               />
             </div>
           }}
        {Styles.spacer(4)}
          <input
            type_="number"
            className=Css.(style([width(px(70))]))
            value={Js.Float.toString(config##rendering##exposure##max)}
            onChange={evt => {
              let v = evt->ReactEvent.Form.target##value->float_of_string;
              let config = [%js.deep
                config["rendering"]["exposure"]["max"].replace(v)
              ];
              update(config, false);
            }}
          />
        </div>
        {Styles.spacer(16)}
        <div className=Styles.row>
          <button
            disabled={config##rendering##exposure##curve == "FourthRoot"}
            onClick={_evt => {
              let config = [%js.deep
                config["rendering"]["exposure"]["curve"].replace("FourthRoot")
              ];
              update(config, false);
            }}
            className=btn>
            {React.string("Fourth Root")}
          </button>
          {Styles.spacer(4)}
          <button
            disabled={config##rendering##exposure##curve == "SquareRoot"}
            onClick={_evt => {
              let config = [%js.deep
                config["rendering"]["exposure"]["curve"].replace("SquareRoot")
              ];
              update(config, false);
            }}
            className=btn>
            {React.string("Square Root")}
          </button>
          {Styles.spacer(4)}
          <button
            disabled={config##rendering##exposure##curve == "Linear"}
            onClick={_evt => {
              let config = [%js.deep
                config["rendering"]["exposure"]["curve"].replace("Linear")
              ];
              update(config, false);
            }}
            className=btn>
            {React.string("Linear")}
          </button>
        </div>
      </div>
    </div>;
  };
};

module TransformEditor = {
  [@react.component]
  let make = (~config, ~update, ~wasm) => {
    <div
      className={Styles.join([
        Styles.control,
        Css.(style([display(`flex), flexDirection(`column)])),
      ])}>
      <div className=Styles.title> {React.string("Scene transforms")} </div>
      <div>
        {React.string("Rotational symmetry: ")}
        <input
          type_="number"
          min=0
          value={config##transform##rotational_symmetry |> string_of_int}
          max="30"
          onChange={evt => {
            let v = int_of_string(evt->ReactEvent.Form.target##value);
            let config = [%js.deep
              config["transform"]["rotational_symmetry"].replace(v)
            ];
            update(config, false);
          }}
        />
      </div>
      {Styles.spacer(8)}
      <div>
        <input
          type_="checkbox"
          checked={config##transform##reflection}
          onChange={evt => {
            let checked = evt->ReactEvent.Form.target##checked;
            let config = [%js.deep
              config["transform"]["reflection"].replace(checked)
            ];
            update(config, false);
          }}
        />
        {React.string(" Reflect over y axis")}
      </div>
      {Styles.spacer(8)}
      <div>
        {React.string("Center offset: ")}
        <input
          type_="number"
          className=Css.(style([width(px(50))]))
          value={config##rendering##center |> fst |> Js.Float.toString}
          onChange={evt => {
            let x = float_of_string(evt->ReactEvent.Form.target##value);
            let config = [%js.deep
              config["rendering"]["center"].map(((_, y)) => (x, y))
            ];
            update(config, false);
          }}
        />
        <input
          type_="number"
          value={config##rendering##center |> snd |> Js.Float.toString}
          className=Css.(style([width(px(50))]))
          onChange={evt => {
            let y = float_of_string(evt->ReactEvent.Form.target##value);
            let config = [%js.deep
              config["rendering"]["center"].map(((x, _)) => (x, y))
            ];
            update(config, false);
          }}
        />
      </div>
      {Styles.spacer(8)}
      <div>
        {React.string("Zoom: ")}
        <input
          type_="number"
          value={config##rendering##zoom |> Js.Float.toString}
          className=Css.(style([width(px(50))]))
          onChange={evt => {
            let zoom = float_of_string(evt->ReactEvent.Form.target##value);
            let config = [%js.deep config["rendering"]["zoom"].replace(zoom)];
            update(config, false);
          }}
        />
      </div>
      {Styles.spacer(8)}
      <ExposureFunction wasm config update />
    </div>;
  };
};

let genId = () =>
  Js.Math.random()
  ->Js.Float.toStringWithRadix(~radix=36)
  ->Js.String2.sliceToEnd(~from=2);
let genId = () => genId() ++ genId();

let newScene = scene => {
  let id = genId();
  let created = Js.Date.now();
  let fullId = created->Js.Float.toString ++ ":" ++ id;
  {...scene, id: fullId, modified: created, created};
};

/**
Behavior:
- on first load, get the data from the hash, might be async
- on hash change that I initiate,
- to detect hashchanges I don't initiate, should use a ref probably. Update the ref then set the hash
  */

let hashIt: string => string = [%bs.raw
  {|
function(input) {
    var hash = 0;
    if (input.length == 0) {
        return hash;
    }
    for (var i = 0; i < input.length; i++) {
        var char = input.charCodeAt(i);
        hash = ((hash<<5)-hash)+char;
        hash = hash & hash; // Convert to 32bit integer
    }
    return hash;
}
|}
];

let anyHash = data => {
  hashIt(Js.Json.stringifyAny(data)->Opt.force);
};

let debounced = (fn, time) => {
  let timer = ref(None);
  arg => {
    switch (timer^) {
    | None => ()
    | Some(timer) => Js.Global.clearTimeout(timer)
    };

    timer := Some(Js.Global.setTimeout(() => fn(arg), time));
  };
};

let interestingDefault: Rust.config = [%bs.raw
  {|
{"walls":[{"kind":{"Parabola":{"a":-0.00297538,"left":-87.706406,"right":37.56985,"transform":{"rotation":[-0.92809707,0.37233835],"translation":[46.11507,-51.88139]}}},"properties":{"absorb":0,"reflect":0,"roughness":0,"refraction":0.35065687},"hide":false},{"kind":{"Line":{"a":[164,-18],"b":[113.16666,-11.666672]}},"properties":{"absorb":0,"reflect":1,"roughness":0,"refraction":0.5},"hide":false},{"kind":{"Circle":[{"radius":25.632011},[210,9],-1.2120256,1.2890245]},"properties":{"absorb":0,"reflect":0.45,"roughness":0,"refraction":0.45742485},"hide":false}],"lights":[{"kind":{"Point":{"offset":0,"origin":[0,0],"t0":-3.1415927,"t1":3.1415927}},"brightness":1}],"transform":{"rotational_symmetry":5,"reflection":true},"rendering":{"exposure":{"curve":"SquareRoot","min":0.028320312,"max":0.38378906},"coloration":{"Rgb":{"background":[0,0,0],"highlight":[255,242,217]}},"width":1024,"height":576,"center":[1,0],"zoom":1}}
|}
];

module Router = {
  let loadHash = (~hash, ~wasm: Rust.wasm, ~onLoad) =>
    if (Js.String2.startsWith(hash, "#id=")) {
      let id = Js.String2.sliceToEnd(hash, ~from=4);
      let%Async.Consume config = Web.LocalForage.getItem(id);
      switch (config->Js.toOption) {
      | None => ()
      | Some(config) => onLoad((Some(id), config))
      };
    } else if (String.length(hash) > 1) {
      let config =
        wasm##parse_url_config(hash->Js.String2.sliceToEnd(~from=1))
        ->Js.toOption;
      switch (config) {
      | None => ()
      | Some(config) => onLoad((None, config))
      };
    } else {
      // ermm maybe not a reset? idk.
      onLoad((None, interestingDefault));
    };

  let updateId = (set, id) => {
    let hash = "id=" ++ id;
    set(hash);
    Web.Location.setHash(hash);
  };
  let permalink = (set, hash) => {
    set(hash);
    Web.Location.setHash(hash);
  };

  let useRouter = (~wasm: Rust.wasm, ~onLoad) => {
    let prevHash = React.useRef(None);
    let hash = Hooks.useHash();
    React.useEffect2(
      () => {
        if (prevHash->React.Ref.current != Some(hash)) {
          prevHash->React.Ref.setCurrent(Some(hash));
          loadHash(~hash, ~wasm, ~onLoad);
        };
        None;
      },
      (hash, prevHash->React.Ref.current),
    );

    React.useCallback(newHash =>
      prevHash->React.Ref.setCurrent(Some(newHash))
    );
    // updateId = id => {
    //   let hash = "id=" ++ id;
    //   prevHash->React.Ref.setCurrent(Some(hash));
    //   Web.Location.setHash(hash);
    // }
    // ~permalink=hash => {
    //   prevHash->React.Ref.setCurrent(Some(hash));
    //   Web.Location.setHash(hash);
    // },
  };
};

let downloadCanvas: (string, Web.canvas) => unit = [%bs.raw
  {|
(function(title, canvas) {
  var a = document.createElement('a');
  a.download = title + ".png";
  a.href = canvas.toDataURL('image/png')
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
})
|}
];

let downloadFile: (string, string) => unit = [%bs.raw
  {|
(function(title, data) {
  var a = document.createElement('a');
  a.download = title + ".json";
  a.href = 'data:application/json,' + data;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
})
|}
];

Css.(
  global(
    "body",
    [
      backgroundColor(Colors.background),
      color(Colors.text),
      fontSize(px(12)),
      margin(px(0)),
    ],
  )
);

Css.(
  global("input", [backgroundColor(Colors.control), color(Colors.text)])
);

Css.(
  global(
    "button",
    [
      backgroundColor(Colors.button),
      cursor(`pointer),
      color(Colors.text),
      padding2(~v=px(4), ~h=px(8)),
      borderStyle(`none),
      borderRadius(px(4)),
      hover([backgroundColor(Colors.buttonHover)]),
      disabled([backgroundColor(`transparent), cursor(`default)]),
    ],
  )
);

module Inner = {
  type state = {
    directory,
    hoverUrl: option(string),
    current: option(string),
    config: Rust.config,
    ui: Rust.ui,
  };

  [@react.component]
  let make = (~wasm: Rust.wasm, ~directory, ~blank) => {
    let router = ref(_ignored => ());

    let (state, dispatch) =
      React.useReducer(
        (state, action) =>
          switch (action) {
          | `Hover(url) => {...state, hoverUrl: Some(url)}
          | `Unhover => {...state, hoverUrl: None}
          | `Route(parentId, parentConfig) => {
              ...state,
              current: parentId,
              config: parentConfig,
            }
          | `UpdateTags(tags) =>
            // TODO safe the tags too yall
            {
              ...state,
              directory: {
                ...state.directory,
                tags,
              },
            }
          | `SaveInPlace((scene: scene)) => {
              ...state,
              directory: {
                ...state.directory,
                scenes:
                  state.directory.scenes
                  ->Belt.Map.String.set(scene.id, scene),
              },
            }
          | `Save((scene: scene)) =>
            (router^)->Router.updateId(scene.id);
            {
              ...state,
              directory: {
                ...state.directory,
                scenes:
                  state.directory.scenes
                  ->Belt.Map.String.set(scene.id, scene),
              },
              current: Some(scene.id),
            };
          | `Update(config, ui) => {...state, config, ui}
          | `Select(id) =>
            (router^)->Router.updateId(id);
            // updateId(id);
            {...state, current: Some(id)};
          },
        {
          directory,
          current: None,
          config: blank,
          ui: Rust.blankUi,
          hoverUrl: None,
        },
      );

    React.useEffect0(() => {
      // Js.log3("Setting up here", anyHash(state.config), state.config);
      let update =
        debounced(
          ((config, ui)) =>
            // configRef->React.Ref.setCurrent(config);
            dispatch(`Update((config, ui))),
          200,
        );
      wasm##setup(state.config, (config, ui)
        // Prevent a render loop
        // Js.log("Setting current from wasm (TODO debounce)");
        // configRef->React.Ref.setCurrent(config);
        // dispatch(`Update(config))
        => update((config, ui)));
      None;
    });

    router :=
      Router.useRouter(
        ~wasm,
        ~onLoad=((id, config)) => {
          // Js.log("Router log");
          // This upgrades the schema if needed
          let config = wasm##restore(config);
          dispatch(`Route((id, config)));
        },
      );

    Hooks.useOnChange(
      state.directory,
      directory => {
        Js.log("Directory changed -- saving");
        let%Async.Consume () = ScenePicker.saveSceneInfo(directory);
        ();
      },
    )
    ->ignore;

    let onSaveScene =
      React.useCallback1(
        (scene: scene) => {
          let scene =
            scene.id == ""
              ? newScene(scene) : {...scene, modified: Js.Date.now()};
          let canvas = Web.documentGetElementById("drawing")->Web.asCanvas;
          canvas->Web.toBlob(blob => {
            let%Async.Consume () =
              Web.LocalForage.setItem(scene.id ++ ":image", blob);
            let%Async.Consume () =
              Web.LocalForage.setItem(scene.id, state.config);
            dispatch(`Save(scene));
            ();
          });
        },
        [|state.config|],
      );

    let update = (config, checkpoint) => {
      wasm##update(config, checkpoint);
      dispatch(`Update((config, state.ui)));
    };

    <div
      className=Css.(
        style([
          display(`flex),
          flexDirection(`row),
          alignItems(`stretch),
          maxWidth(px(1300)),
          margin2(~v=`zero, ~h=`auto),
          height(`vh(100.0)),
          // flexWrap(`wrap),
        ])
      )>
      <div
        className=Css.(
          style([
            position(`relative),
            display(`flex),
            flexDirection(`column),
            // overflow(`hidden),
            alignItems(`stretch),
          ])
        )>
        <div className=Css.(style([flexShrink(0), position(`relative)]))>
          <canvas
            id="drawing"
            width="600"
            height="600"
            className=Css.(style([]))
          />
          {switch (state.hoverUrl) {
           | None => React.null
           | Some(url) =>
             <img
               src=url
               className=Css.(
                 style([
                   backgroundColor(black),
                   position(`absolute),
                   top(px(0)),
                   pointerEvents(`none),
                   left(px(0)),
                 ])
               )
             />
           }}
        </div>
        <MiniScenePicker
          directory={state.directory}
          onChangeScene={scene => dispatch(`SaveInPlace(scene))}
          onClearScene={() => {
            let _ = wasm##restore(wasm##blank_config());
            ();
          }}
          onSaveScene
          current={state.current}
          hover={url => dispatch(`Hover(url))}
          unHover={() => dispatch(`Unhover)}
          onSelect={(id, config) => {
            Js.log3("Resetting", anyHash(config), config);
            let _config = wasm##restore(config);
            (router^)->Router.updateId(id);
            dispatch(`Select(id));
          }}
        />
      </div>
      <div
        className=Css.(
          style([
            margin2(~h=px(8), ~v=px(8)),
            flex(1),
            display(`flex),
            flexDirection(`column),
            minHeight(px(200)),
          ])
        )>
        {
          let currentScene =
            switch (state.current) {
            | None => None
            | Some(key) => state.directory.scenes->Belt.Map.String.get(key)
            };

          <SceneForm
            directory={state.directory}
            onUpdateTags={tags => dispatch(`UpdateTags(tags))}
            scene=?currentScene
            wasm
            onPermalink={() =>
              (router^)
              ->Router.permalink(wasm##serialize_url_config(state.config))
            }
            onDownload={() => {
              let title =
                switch (currentScene) {
                | Some({title: Some(title)}) => title
                | Some({created}) =>
                  Js.Date.toISOString(Js.Date.fromFloat(created))
                | _ => Js.Date.toISOString(Js.Date.make())
                };

              // let data = Js.Json.stringifyAny(state.config);
              // switch (data) {
              // | None => ()
              // | Some(data) =>
              //   downloadFile(title, data);
              // };
              downloadCanvas(
                title,
                Web.documentGetElementById("drawing")->Web.asCanvas,
              );
            }}
            onSave={scene => onSaveScene(scene)}
            key={
              switch (currentScene) {
              | None => "new-scene"
              | Some(scene) => scene.id
              }
            }
          />;
        }
        {Styles.spacer(8)}
        <TransformEditor
          wasm
          config={state.config}
          update={(config, checkpoint) => {
            wasm##update(config, checkpoint);
            dispatch(`Update((config, state.ui)));
          }}
        />
        <Objects
          config={state.config}
          ui={state.ui}
          update={(config, checkpoint) => {
            wasm##update(config, checkpoint);
            dispatch(`Update((config, state.ui)));
          }}
          wasm
          updateUi={ui => {
            Js.log2("New ui", ui);
            wasm##update_ui(ui);
            dispatch(`Update((state.config, ui)));
          }}
        />
      </div>
    </div>;
  };
};

module App = {
  [@react.component]
  let make = (~wasm: Rust.wasm) => {
    let keys = Hooks.useLoading(ScenePicker.getSceneInfo);

    switch (keys) {
    | None => <div> {React.string("Loading")} </div>
    | Some(directory) => <Inner wasm directory blank={wasm##blank_config()} />
    };
  };
};

Rust.withModule(wasm
  // wasm##run();
  // let config = wasm##save();
  =>
    ReactDOMRe.renderToElementWithId(
      <App wasm />,
      "reason-root",
      // Js.log2("Config we got", config);
    )
  );
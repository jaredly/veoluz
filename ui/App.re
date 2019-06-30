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

[%bs.raw{|require('rc-color-picker/assets/index.css')|}]

open Types;

module ExposureFunction = {
  [@react.component]
  let make = (~config, ~update) => {
    <div>
      {React.string("Exposure function: ")}
      <button
        disabled={config##rendering##exposure##curve == "FourthRoot"}
        onClick={_evt => {
          let config = [%js.deep
            config["rendering"]["exposure"]["curve"].replace("FourthRoot")
          ];
          update(config, false);
        }}
        className="btn"
      >
        {React.string("Fourth Root")}
      </button>
      <button
        disabled={config##rendering##exposure##curve == "SquareRoot"}
        onClick={_evt => {
          let config = [%js.deep
            config["rendering"]["exposure"]["curve"].replace("SquareRoot")
          ];
          update(config, false);
        }}
        className="btn"
      >
        {React.string("Square Root")}
      </button>
      <button
        disabled={config##rendering##exposure##curve == "Linear"}
        onClick={_evt => {
          let config = [%js.deep
            config["rendering"]["exposure"]["curve"].replace("Linear")
          ];
          update(config, false);
        }}
        className="btn"
      >
        {React.string("Linear")}
      </button>
    </div>

  };
}

module TransformEditor = {
  [@react.component]
  let make = (~config, ~update) => {
    <div>
      {React.string("Rotational symmetry: ")}
      <input
        type_="number"
        min=0
        value={config##transform##rotational_symmetry |> string_of_int}
        max="30"
        onChange={evt => {
          let v = int_of_string((evt->ReactEvent.Form.target)##value);
          let config = [%js.deep
            config["transform"]["rotational_symmetry"].replace(v)
          ];
          update(config, false);
        }}
      />
      <br/>
      <input
        type_="checkbox"
        checked={config##transform##reflection}
        onChange={evt => {
          let checked = (evt->ReactEvent.Form.target)##checked;
          let config = [%js.deep
            config["transform"]["reflection"].replace(checked)
          ];
          update(config, false);
        }}
      />
      {React.string(" Reflect over y axis")}
    </div>
  }
}

module ConfigEditor = {
  [@react.component]
  let make = (~config: Rust.config, ~wasm, ~update, ~onSaveScene) => {
    let (tmpConfig, setTmpConfig) = Hooks.useUpdatingState(config);

    <div>
      <div>
        <ExposureControl wasm config update />
        <ExposureFunction config update />
        <TransformEditor config update />
      </div>
      // <div> {React.string(Js.Json.stringifyWithSpace(Obj.magic(tmpConfig), 2))} </div>
      <button onClick={_ => onSaveScene()}>
        {React.string("Save Sceen")}
      </button>
    </div>;
  };
};

let genId = () =>
  Js.Math.random()
  ->Js.Float.toStringWithRadix(~radix=36)
  ->Js.String2.sliceToEnd(~from=2);
let genId = () => genId() ++ genId();

let newScene = () => {
  let id = genId();
  let created = Js.Date.now();
  let fullId = created->Js.Float.toString ++ ":" ++ id;
  {
    id: fullId,
    modified: created,
    created,
    title: None,
    tags: Belt.Set.String.empty,
    children: [||],
    parent: None,
  };
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
      onLoad((None, wasm##blank_config()));
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

module Inner = {
  type state = {
    directory,
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
          | `Route(parentId, parentConfig) => {
              ...state,
              current: parentId,
              config: parentConfig,
            }
          | `Save((scene: scene)) => {
              ...state,
              directory: {
                ...state.directory,
                scenes:
                  state.directory.scenes
                  ->Belt.Map.String.set(scene.id, scene),
              },
              current: Some(scene.id),
            }
          | `Update(config, ui) => {...state, config, ui}
          | `Select(id) =>
            (router^)->Router.updateId(id);
            // updateId(id);
            {...state, current: Some(id)};
          },
        {directory, current: None, config: blank, ui: Rust.blankUi},
      );

    React.useEffect0(() => {
      // Js.log3("Setting up here", anyHash(state.config), state.config);
      let update =
        debounced(
          ((config, ui)) =>
            // configRef->React.Ref.setCurrent(config);
            dispatch(
              `Update(config, ui),
            ),
          200,
        );
      wasm##setup(state.config, (config, ui) =>
        // Prevent a render loop
        // Js.log("Setting current from wasm (TODO debounce)");
        // configRef->React.Ref.setCurrent(config);
        // dispatch(`Update(config))
        update(
          (config, ui)
        )
      );
      None;
    });

    router :=
      Router.useRouter(
        ~wasm,
        ~onLoad=((id, config)) => {
          // Js.log("Router log");
          wasm##restore(config);
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
        () => {
          let scene = newScene();
          let canvas = Web.documentGetElementById("drawing")->Web.asCanvas;
          canvas->Web.toBlob(blob => {
            let%Async.Consume () = Web.LocalForage.setItem(scene.id ++ ":image", blob);
            let%Async.Consume () = Web.LocalForage.setItem(scene.id, state.config);
            dispatch(`Save(scene));
            ();
          });
        },
        [|state.config|],
      );

    <div>
      <div className=Css.(style([
        display(`flex),
        flexDirection(`row),
        flexWrap(`wrap),
      ]))>
        <canvas id="drawing" width="600" height="600" />
        <div className=Css.(style([
          marginLeft(px(16))
        ]))>
          <Objects config=state.config ui=state.ui update={(config, checkpoint) => {
            wasm##update(config, checkpoint);
            dispatch(`Update(config, state.ui));
          }} wasm />
        </div>
      </div>
      <ConfigEditor
        wasm
        config={state.config}
        onSaveScene
        update={(config, checkpoint) => {
          // configRef->React.Ref.setCurrent(config);
          wasm##update(config, checkpoint);
          dispatch(`Update(config, state.ui));
        }}
      />
      <ScenePicker
        directory={state.directory}
        current={state.current}
        onSelect={(id, config) => {
          Js.log3("Resetting", anyHash(config), config);
          // configRef->React.Ref.setCurrent(config);
          wasm##restore(config);
          (router^)->Router.updateId(id);
          dispatch(`Select(id));
        }}
      />
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
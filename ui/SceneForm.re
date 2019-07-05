open Belt;

let item =
  Css.(
    style([
      padding(px(4)),
      cursor(`pointer),
      hover([backgroundColor(hex("eee"))]),
    ])
  );

let selected =
  Styles.join([item, Css.(style([backgroundColor(hex("aaa"))]))]);

module DropDown = {
  type state('a) = {
    value: string,
    selection: option(int),
    items: array('a),
  };
  let reduce = (state, action) =>
    switch (action) {
    | `Type(value, getList) => {
        value,
        selection: Some(0),
        items: getList(value),
      }
    | `Down => {
        ...state,
        selection:
          switch (state.selection) {
          | None => None
          | Some(i) => i < state.items->Array.length ? Some(i + 1) : Some(0)
          },
      }
    | `Up => {
        ...state,
        selection:
          switch (state.selection) {
          | None => None
          | Some(i) => i > 0 ? Some(i - 1) : Some(state.items->Array.length)
          },
      }
    | `Open => {...state, selection: Some(0)}
    | `Close => {...state, selection: None}
    };
  [@react.component]
  let make = (~onSelect, ~onCreate, ~getList, ~render) => {
    let (state, dispatch) =
      React.useReducer(reduce, {value: "", selection: None, items: getList("")});
    <div className=Css.(style([position(`relative)]))>
      <input
        value={state.value}
        onKeyDown={evt =>
          switch (evt->ReactEvent.Keyboard.key) {
          | "ArrowUp" => dispatch(`Up)
          | "ArrowDown" => dispatch(`Down)
          | "Return"
          | "Enter" =>
            evt->ReactEvent.Keyboard.preventDefault;
            dispatch(`Close);
            switch (state.selection) {
            | None => ()
            | Some(i) =>
              if (i == state.items->Array.length) {
                onCreate(state.value);
              } else {
                switch (state.items[i]) {
                | None => ()
                | Some(tag) => onSelect(tag)
                };
              }
            };
          | _ => ()
          }
        }
        onChange={evt => {
          let text = evt->ReactEvent.Form.target##value;
          dispatch(`Type((text, getList)));
        }}
        onFocus={_ => dispatch(`Open)}
        onBlur={_ => dispatch(`Close)}
      />
      {switch (state.selection) {
       | None => React.null
       | Some(selection) =>
         <div
           className=Css.(
             style([
               position(`absolute),
               top(`percent(100.0)),
               backgroundColor(red),
               padding(px(8)),
               left(px(0)),
             ])
           )>
           {state.items
            ->Belt.Array.map(item =>
                render(
                  item,
                  () => {
                    dispatch(`Close);
                    onSelect(item);
                  },
                )
              )
            ->React.array}
           <div
             className={
               selection == state.items->Array.length ? selected : item
             }
             onClick={_ => {
               dispatch(`Close);
               onCreate(state.value);
             }}>
             {React.string("Create new: " ++ state.value)}
           </div>
         </div>
       }}
    </div>;
  };
};

module TagsEditor = {
  open Types;

  [@react.component]
  let make = (~directory, ~tags, ~onChange, ~onUpdateTags) => {
    <div>
      {tags
       ->Belt.Set.String.toArray
       ->Belt.Array.map(tid =>
           switch (directory.tags->Belt.Map.String.get(tid)) {
           | None => React.null
           | Some(tag) => <div> {React.string(tag.title)} </div>
           }
         )
       ->React.array}
      <DropDown
        getList={text =>
          directory.tags
          ->Belt.Map.String.valuesToArray
          ->Belt.Array.keep(t => t.title->Js.String2.includes(text))
        }
        render={(tag, onClick) =>
          <div
            key={tag.id}
            className=item
            onMouseDown={_evt => onClick()}>
            {React.string(tag.title)}
          </div>
        }
        onSelect={tag => onChange(tags->Belt.Set.String.add(tag.id))}
        onCreate={title => {
          let id = Types.genId();
          onUpdateTags(
            directory.tags
            ->Belt.Map.String.set(id, {id, color: "white", title}),
          );
        }}
      />
    </div>;
  };
};

[@react.component]
let make =
    (
      ~directory,
      ~onUpdateTags,
      ~scene=Types.emptyScene,
      ~onSave,
      ~onPermalink,
      ~onDownload,
      ~wasm: Rust.wasm,
    ) => {
  let (scene, update) = Hooks.useState(scene);
  <div
    className={
      Styles.control
      ++ " "
      ++ Css.(
           style([
             flexDirection(`column),
             display(`flex),
             alignItems(`stretch),
           ])
         )
    }>
    <div className=Styles.title>
      {React.string(
         scene.id == ""
           ? "New scene"
           : "Scene created "
             ++ Js.Date.toLocaleString(Js.Date.fromFloat(scene.created)),
       )}
    </div>
    <input
      className=Css.(style([alignSelf(`stretch)]))
      placeholder="Title"
      value={
        switch (scene.title) {
        | None => ""
        | Some(x) => x
        }
      }
      onChange={evt => {
        let title = evt->ReactEvent.Form.target##value;
        update({...scene, title: title == "" ? None : Some(title)});
      }}
    />
    {Styles.spacer(8)}
    <TagsEditor
      directory
      onUpdateTags
      onChange={tags => update({...scene, tags})}
      tags={scene.tags}
    />
    {Styles.spacer(8)}
    <div className=Styles.row>
      {scene.id != ""
         ? <button onClick={_evt => onSave(scene)}>
             {React.string("Update scene")}
           </button>
         : React.null}
      {Styles.spacer(4)}
      <button onClick={_evt => onSave({...scene, id: ""})}>
        {React.string(scene.id == "" ? "Save scene" : "Save new scene")}
      </button>
    </div>
    {Styles.spacer(4)}
    <div>
      <button
        onClick={_ => onPermalink()}
        className={Styles.flatButton(Colors.text)}>
        {React.string("Permalink")}
      </button>
      <button
        onClick={_ => onDownload()}
        className={Styles.flatButton(Colors.text)}>
        {React.string("Download as json")}
      </button>
    </div>
    {Styles.spacer(8)}
    <div>
      <button onClick={evt => wasm##undo()}> {React.string("Undo")} </button>
      <button onClick={evt => wasm##redo()}> {React.string("Redo")} </button>
    </div>
  </div>;
};
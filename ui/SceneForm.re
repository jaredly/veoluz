open Belt;

[@react.component]
let make =
    (
      ~directory,
      ~onUpdateTags,
      ~ui: Rust.ui,
      ~scene=Types.emptyScene,
      ~onSave,
      ~onPermalink,
      ~onDownload,
      ~onDownloadZip,
      ~wasm: Rust.wasm,
    ) => {
  let (scene, update) = Hooks.useState(scene);
  // Styles.control
  // ++ " "
  // ++
  <div
    className=Css.(
      style([flexDirection(`column), display(`flex), alignItems(`stretch)])
    )>
    // <div className=Styles.title>
    //   {React.string(
    //      scene.id == ""
    //        ? "New scene"
    //        : "Scene created "
    //          ++ Js.Date.toLocaleString(Js.Date.fromFloat(scene.created)),
    //    )}
    // </div>
    // <input
    //   className=Css.(style([alignSelf(`stretch)]))
    //   placeholder="Title"
    //   value={
    //     switch (scene.title) {
    //     | None => ""
    //     | Some(x) => x
    //     }
    //   }
    //   onChange={evt => {
    //     let title = evt->ReactEvent.Form.target##value;
    //     update({...scene, title: title == "" ? None : Some(title)});
    //   }}
    // />
    // {Styles.spacer(8)}
    // <TagsEditor
    //   directory
    //   onUpdateTags
    //   onChange={tags => update({...scene, tags})}
    //   tags={scene.tags}
    // />
    // {Styles.spacer(8)}
    // <div className=Styles.row>
    //   {scene.id != ""
    //      ? <button onClick={_evt => onSave(scene)}>
    //          {React.string("Update scene")}
    //        </button>
    //      : React.null}
    //   {Styles.spacer(4)}
    //   <button onClick={_evt => onSave({...scene, id: ""})}>
    //     {React.string(scene.id == "" ? "Save scene" : "Save new scene")}
    //   </button>
    // </div>
    // {Styles.spacer(4)}

      <div className=Styles.row>
        <Tippy content="Undo">
          <button onClick={_evt => wasm##undo()}> <IonIcons.Undo /> </button>
        </Tippy>
        {Styles.spacer(4)}
        <Tippy content="Redo">
          <button onClick={_evt => wasm##redo()}> <IonIcons.Redo /> </button>
        </Tippy>
        {Styles.spacer(4)}
        <Tippy content="Permalink">
          <button onClick={_ => onPermalink()}> <IonIcons.Link /> </button>
        </Tippy>
        {Styles.spacer(4)}
        <Tippy content="Download image">
          <button onClick={_ => onDownload()}> <IonIcons.Download /> </button>
        </Tippy>
        {Styles.spacer(4)}
        <Tippy content="Download image + json config">
          <button onClick={_ => onDownloadZip()}>
            <IonIcons.Compress />
          </button>
        </Tippy>
      </div>
    </div>;
  // {Styles.spacer(4)}
  // <button
  //   className=Css.(
  //     style([
  //       backgroundColor(
  //         ui##show_lasers ? Colors.accent : Colors.button,
  //       ),
  //     ])
  //   )
  //   onClick={_evt =>
  //     wasm##update_ui(
  //       [%js.deep ui["show_lasers"].replace(!ui##show_lasers)],
  //     )
  //   }>
  //   <IonIcons.Flashlight />
  // </button>
};
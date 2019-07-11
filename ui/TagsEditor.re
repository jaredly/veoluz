open Belt;

let item =
  Css.(
    style([
      padding(px(4)),
      cursor(`pointer),
      hover([backgroundColor(hex("eee"))]),
    ])
  );

let selectedItem =
  Styles.join([item, Css.(style([backgroundColor(Colors.button)]))]);

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
    | `UpdateList(items) => {...state, items}
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
    | `Done(getList) => {...state, value: "", items: getList("")}
    };

  [@react.component]
  let make = (~onSelect, ~onCreate, ~getList, ~render) => {
    let (state, dispatch) =
      React.useReducer(
        reduce,
        {value: "", selection: None, items: getList("")},
      );

    React.useEffect1(
      () => {
        dispatch(`UpdateList(getList(state.value)));
        None;
      },
      [|getList|],
    );

    <div className=Css.(style([position(`relative)]))>
      <input
        value={state.value}
        onKeyDown={evt =>
          switch (evt->ReactEvent.Keyboard.key) {
          | "ArrowUp" => dispatch(`Up)
          | "ArrowDown" => dispatch(`Down)
          | "Escape" => dispatch(`Close)
          | "Return"
          | "Enter" =>
            evt->ReactEvent.Keyboard.preventDefault;
            dispatch(`Done(getList));
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
        className=Css.(
          style([
            backgroundColor(`transparent),
            color(white),
            padding(px(8)),
            borderStyle(`none),
          ])
        )
        placeholder="Add a tag"
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
               zIndex(1000),
               position(`absolute),
               top(`percent(100.0)),
               maxHeight(px(200)),
               overflow(`auto),
               backgroundColor(white),
               boxShadow(~blur=px(5), Colors.accent),
               padding(px(8)),
               left(px(0)),
             ])
           )>
           {state.items
            ->Belt.Array.mapWithIndex((i, item) =>
                render(
                  ~selected=i == selection,
                  item,
                  () => {
                    dispatch(`Done(getList));
                    onSelect(item);
                  },
                )
              )
            ->React.array}
           <div
             className={
               selection == state.items->Array.length ? selectedItem : item
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

open Types;

[@react.component]
let make =
    (
      ~sceneTags,
      ~tags,
      ~onChange,
      ~onUpdateTags,
      ~onClickTag,
      ~highlightedTags,
    ) => {
  let getList =
    React.useCallback2(
      text =>
        tags
        ->Belt.Map.String.valuesToArray
        ->Belt.Array.keep(t =>
            !sceneTags->Belt.Set.String.has(t.id)
            && t.title->Js.String2.includes(text)
          )
        ->Js.Array2.sortInPlaceWith((t1, t2) =>
            t1.title->Js.String2.indexOf(text)
            - t2.title->Js.String2.indexOf(text)
          ),
      (sceneTags, tags),
    );

  <div className=Styles.row>
    <IonIcons.Tag color="white" className=Css.(style([margin(px(8))])) />
    <div
      className={Styles.join([
        Styles.row,
        Css.(style([flexWrap(`wrap), flexShrink(1)])),
      ])}>
      {sceneTags
       ->Belt.Set.String.toArray
       ->Belt.Array.map(tid =>
           switch (tags->Belt.Map.String.get(tid)) {
           | None => React.null
           | Some(tag) =>
             <div
               onClick={_ => onClickTag(tag.id)}
               className=Css.(
                 style([
                   flexShrink(0),
                   cursor(`pointer),
                   borderRadius(px(4)),
                   marginTop(px(2)),
                   display(`flex),
                   alignItems(`center),
                   flexDirection(`row),
                   marginBottom(px(2)),
                   marginRight(px(8)),
                   backgroundColor(
                     highlightedTags->Belt.Set.String.has(tid)
                       ? Colors.accent : Colors.button,
                   ),
                   hover([backgroundColor(Colors.accent)]),
                 ])
               )>
               <div className=Css.(style([padding2(~v=px(3), ~h=px(8))]))>
                 {React.string(tag.title)}
               </div>
               <button
                 className={Styles.join([Styles.iconButton])}
                 onClick={evt => {
                   ReactEvent.Mouse.stopPropagation(evt);
                   onChange(sceneTags->Belt.Set.String.remove(tag.id));
                 }}>
                 <IonIcons.Close color="currentcolor" />
               </button>
             </div>
           }
         )
       ->React.array}
    </div>
    {Styles.spacer(8)}
    <DropDown
      getList
      render={(~selected, tag, onClick) =>
        <div
          key={tag.id}
          className={selected ? Styles.join([item, selectedItem]) : item}
          onMouseDown={_evt => onClick()}>
          {React.string(tag.title)}
        </div>
      }
      onSelect={tag => onChange(sceneTags->Belt.Set.String.add(tag.id))}
      onCreate={title => {
        let id = Types.genId();
        onUpdateTags(
          tags->Belt.Map.String.set(id, {id, color: "white", title}),
        );
      }}
    />
  </div>;
};
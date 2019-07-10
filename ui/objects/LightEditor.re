open Ui;

[@react.component]
let make = (~wasm, ~selected, ~light, ~index, ~onChange, ~updateUi, ~ui) => {
  <div
    className=Css.(
      style(
        [
          padding(px(8)),
          cursor(`pointer),
          hover([backgroundColor(Colors.buttonHover)]),
          margin2(~v=px(8), ~h=px(0)),
        ]
        @ (
          selected
            ? [outline(px(2), `solid, Colors.accent), hover([])] : []
        ),
      )
    )
    onMouseOver={evt =>
      // wasm##hover_wall(index)
      ()}>
    <div
      className=Css.(
        style([
          display(`flex),
          alignItems(`center),
          fontWeight(`medium),
          fontSize(px(Styles.Text.small)),
        ])
      )
      onClick={evt => {
        if (selected) {
          updateUi([%js.deep ui["selection"].replace(Js.null)]);
        } else {
          updateUi(
            [%js.deep
              ui["selection"].replace(
                Js.Null.return(Rust.selectLight(index)),
              )
            ],
          );
        };
        ();
      }}>
      {selected
         ? <IonIcons.ArrowDown fontSize="14px" />
         : <IonIcons.ArrowRight fontSize="14px" />}
      {React.string("Light #" ++ string_of_int(index))}
    </div>
    {selected
       ? {
         let point = [%js.deep light##kind["Point"]];
         <div className=Styles.column>
           {Styles.spacer(16)}
           <div>
             {React.string("Radius ")}
             <NumInput
               min=0
               max=500.0
               step=5.0
               value={point##offset}
               onChange={offset =>
                 onChange(
                   [%js.deep
                     light["kind"]["Point"]["offset"].replace(offset)
                   ],
                 )
               }
             />
           </div>
           {Styles.spacer(8)}
           <div className=Styles.row>
             {React.string("x ")}
             {Styles.spacer(4)}
             <NumInput
               min=(-500)
               max=500.0
               step=5.0
               value={point##origin |> fst}
               onChange={x =>
                 onChange(
                   [%js.deep
                     light["kind"]["Point"]["origin"].map(((_, y)) =>
                       (x, y)
                     )
                   ],
                 )
               }
             />
             {Styles.spacer(8)}
             {React.string("y ")}
             {Styles.spacer(4)}
             <NumInput
               min=(-500)
               max=500.0
               step=5.0
               value={point##origin |> snd}
               onChange={y =>
                 onChange(
                   [%js.deep
                     light["kind"]["Point"]["origin"].map(((x, _)) =>
                       (x, y)
                     )
                   ],
                 )
               }
             />
           </div>
           {Styles.spacer(8)}
           <div className=Styles.row>
             {React.string("t0 ")}
             {Styles.spacer(4)}
             <NumInput
               min=0
               max={Js.Math._PI *. 2.0}
               step=0.1
               value={point##t0}
               onChange={t0 =>
                 onChange(
                   [%js.deep light["kind"]["Point"]["t0"].replace(t0)],
                 )
               }
             />
             {Styles.spacer(8)}
             {React.string("t1 ")}
             {Styles.spacer(4)}
             <NumInput
               min=0
               max={Js.Math._PI *. 2.0}
               step=0.1
               value={point##t1}
               onChange={t1 =>
                 onChange(
                   [%js.deep light["kind"]["Point"]["t1"].replace(t1)],
                 )
               }
             />
           </div>
         </div>;
       }
       : React.null}
  </div>;
};
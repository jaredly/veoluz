
let useState = initial => {
  React.useReducer((_, action) => action, initial);
};

let useLoading = getter => {
  let (data, setData) = useState(None);
  React.useEffect1(
    () => {
      if (data != None) {
        setData(None);
      };
      getter()
      |> Js.Promise.then_(result => {
           setData(Some(result));
           Js.Promise.resolve();
         })
      |> ignore;
      None;
    },
    [|getter|],
  );
  data;
};
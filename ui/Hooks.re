
let useState = initial => {
  React.useReducer((_, action) => action, initial);
};

let useUpdatingState = initial => {
  let (current, setState) = useState(initial);
  React.useEffect1(
    () => {
      if (initial !== current) {
        setState(initial);
      };
      None;
    },
    [|initial|],
  );
  (current, setState)
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
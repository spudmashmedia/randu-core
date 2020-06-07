import React, { useState, useEffect } from "react";
import { logger, get_user, LogLevel } from "../lib/wasm";
import Profile from "./Profile";
import { v1 as uuidv1 } from "uuid";

export default function HocUser(props) {
  logger(LogLevel.debug, `[HocUser] inside hoc`);

  const { gender, nationality, limit } = props;

  logger(
    LogLevel.debug,
    `[HocUser] props received gender=${gender} nationality=${nationality} limit=${limit}`
  );

  const [data, setData] = useState();

  useEffect(() => {
    //
    // IIFE (immediately invoked function expression) to get around to executing async
    // wasm function wasm.get_user_wasm
    // otherwise error: "TypeError: func.apply is not a function"
    //
    (async () => {
      try {
        let result = await get_user(gender, nationality, limit);
        setData(result);
      } catch (e) {
        logger(error, `HocUser error ${e}`);
      }
    })();
  }, []);

  return (
    <div>
      <ul>
        {data ? (
          data.map((item) => {
            let { name, username, password, location } = item;
            return (
              <li key={uuidv1()}>
                <Profile name={name} username={username} password={password} location={location} />
              </li>
            );
          })
        ) : (
          <span>Loading...</span>
        )}
      </ul>
    </div>
  );
}

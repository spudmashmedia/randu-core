/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Spudmash Media Pty Ltd
 *  Licensed under the MIT License. See License.md in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import React, { useState, useEffect } from "react";
import Profile from "./Profile";
import { logger, get_user, LogLevel } from "../lib/wasm";
import { v1 as uuidv1 } from "uuid";
import Grid from "@material-ui/core/Grid";
import CircularProgress from "@material-ui/core/CircularProgress";

export default function UserList(props) {
  logger(LogLevel.Debug, `[UserList] enter`);

  // props
  let { gender, nat, limit } = props;

  // hooks
  const [data, setData] = useState([]);

  useEffect(() => {
    get_user(gender, nat, limit)
      .then((result) => setData(result))
      .catch((e) => logger(LogLevel.Error, `HocUser error ${e}`));
  }, [gender, nat, limit]);

  return (
    <Grid container spacing={3}>
      {data ? (
        data.map((item) => {
          return (
            <Grid item key={uuidv1()}>
              <Profile data={item} />
            </Grid>
          );
        })
      ) : (
        <CircularProgress />
      )}
    </Grid>
  );
}

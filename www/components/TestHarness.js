/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Spudmash Media Pty Ltd
 *  Licensed under the MIT License. See License.md in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import React, { useState } from "react";
import { logger, LogLevel, Gender, Nationality } from "../lib/wasm";
import UserList from "./UserList";
import DropDown from "./DropDown";
import Grid from "@material-ui/core/Grid";
import Paper from "@material-ui/core/Paper";
import { makeStyles } from "@material-ui/core/styles";

const useStyles = makeStyles((theme) => ({
  root: {
    flexGrow: 1,
  },
  grid: {
    margin: 5,
  },
  paper: {
    height: 150,
    width: 250,
  },
}));

export default function TestHarness(props) {
  logger(LogLevel.Debug, `[TestHarness] enter`);

  const classes = useStyles();

  // hooks
  const [limit, setLimit] = useState(1);
  const [gender, setGender] = useState(0);
  const [nat, setNat] = useState(0);

  // binding data
  let genderMap = Object.keys(Gender).map((key) => ({
    key: key,
    value: Gender[key],
  }));

  let nationalityMap = Object.keys(Nationality).map((key) => ({
    key: key,
    value: Nationality[key],
  }));

  let limitMap = new Array(10)
    .fill()
    .map((e, i) => ({ key: i + 1, value: i + 1 }));

  // event handlers
  let onGenderChange = (event) => {
    setGender(event.target.value);
  };

  let onLimitChange = (event) => {
    setLimit(event.target.value);
  };

  let onNatChange = (event) => {
    setNat(event.target.value);
  };

  return (
    <div className={classes.root}>
      <Paper>
        <Grid container spacing={3} className={classes.grid}>
          <Grid item>
            <DropDown
              label="Limit"
              data={limitMap}
              selected={limit}
              onChange={onLimitChange}
            />
          </Grid>
          <Grid item>
            <DropDown
              label="Gender"
              data={genderMap}
              selected={gender}
              onChange={onGenderChange}
            />
          </Grid>
          <Grid item>
            <DropDown
              label="Nationality"
              data={nationalityMap}
              selected={nat}
              onChange={onNatChange}
            />
          </Grid>
        </Grid>
      </Paper>
      <Paper>
        <UserList gender={gender} nat={nat} limit={limit} />
      </Paper>
    </div>
  );
}

/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Spudmash Media Pty Ltd
 *  Licensed under the MIT License. See License.md in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import React from "react";

import { makeStyles } from "@material-ui/core/styles";

import MenuItem from "@material-ui/core/MenuItem";
import FormControl from "@material-ui/core/FormControl";
import Select from "@material-ui/core/Select";
import InputLabel from "@material-ui/core/InputLabel";

const useStyles = makeStyles((theme) => ({
  formControl: {
    margin: theme.spacing(1),
    minWidth: 120,
  },
}));

export default function DropDown(props) {
  let { label } = props;
  let classes = useStyles;

  let { data, selected, onChange } = props;

  return (
    <div>
      <FormControl className={classes.formControl}>
        <InputLabel>{label}</InputLabel>
        <Select value={selected} onChange={onChange}>
          {data.map((items) => (
            <MenuItem key={items.value} value={items.value}>
              {items.key}
            </MenuItem>
          ))}
        </Select>
      </FormControl>
    </div>
  );
}

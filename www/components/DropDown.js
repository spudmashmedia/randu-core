/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Spudmash Media Pty Ltd
 *  Licensed under the MIT License. See License.md in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { useState, useEffect } from "react";
import { getGenderArray, getNationalityArray } from "../lib/wasm";
import { v1 as uuidv1 } from "uuid";

export default function DropDown(props) {
  let { label, data } = props;

  console.log(`DropDown: ${JSON.stringify(data, null, 2)}`);

  let moo = [];
  console.debug(`Nationality array\n${JSON.stringify(data, null, 2)}`);

  return (
    <div>
      <label>{label}</label>
      <select id={label}>
        {moo.forEach(({ id, name }) => {
          <option onLoad={onLoadHandler} key={uuidv1()} value={id}>
            {name}
          </option>;
        })}
      </select>
    </div>
  );
}

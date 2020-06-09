/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Spudmash Media Pty Ltd
 *  Licensed under the MIT License. See License.md in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import React, { useState, useEffect } from "react";

export default function Profile(props) {
  let { name, username, password, location } = props;
  return (
    <div>
      <span>{name}</span>,<span>{username}</span>,<span>{password}</span>,
      <span>{location}</span>
    </div>
  );
}

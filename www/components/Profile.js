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

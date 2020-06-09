/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Spudmash Media Pty Ltd
 *  Licensed under the MIT License. See License.md in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import dynamic from "next/dynamic";
import { useEffect, useState } from "react";

const HocUser = dynamic(() => import("../components/HocUser"));
const DropDown = dynamic(() => import("../components/DropDown"));

function HomePage() {
  let [limit, setLimit] = useState(1);
  let [gender, setGender] = useState("female");
  let [nat, setNat] = useState("au");
  let [natlist, setNatlist] = useState([]);

  useEffect(() => {
    setLimit(1);
    setGender(0);
    setNat(6);

    console.log(
      `homepage items set: gender=${gender} nat=${nat} limit=${limit}`
    );
  });

  return (
    <div>
      <DropDown label={"Nationality"} data={natlist}></DropDown>
      <HocUser gender={gender} nationality={nat} limit={limit} />
    </div>
  );
}

export default HomePage;

/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Spudmash Media Pty Ltd
 *  Licensed under the MIT License. See License.md in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import dynamic from "next/dynamic";
import { useEffect, useState } from "react";

const TestHarness = dynamic(() => import("../components/TestHarness"));


function HomePage() {

  return (
    <div>
      <TestHarness />
    </div>
  );
}

export default HomePage;

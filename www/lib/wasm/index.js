import {
  LogLevel,
  ApplicationType,
  Gender,
  Nationality,
  wasm_logger,
  get_user_wasm,
} from "randu-core";

function logger(logLevel, msg) {
  wasm_logger(logLevel, ApplicationType.ClientWeb, msg);
}

async function get_user(gender, nationality, limit) {
  try {
    logger(LogLevel.debug, `inside lib get_user gender=${gender} nat=${nationality} limit=${limit}`);

    let result = await get_user_wasm(gender, nationality, limit);

    logger(
      LogLevel.debug,
      `lib get_user result:\n${JSON.stringify(result, null, 2)}`
    );

    return result;
  } catch (error) {
    logger(LogLevel.error, `[HocUser] get_user_wasm error:\n${error}`);
  }
}

let enumToArray = (val) => {
  const arrayObjects = [];
  for (const [propertyKey, propertyValue] of Object.entries(val)) {
    if (!Number.isNaN(Number(propertyKey))) {
      continue;
    }
    arrayObjects.push({ id: propertyValue, name: propertyKey });
  }
  return arrayObjects;
};

const getGenderArray = () => enumToArray(Gender);
const getNationalityArray = () => enumToArray(Nationality);

export {
  get_user,
  logger,
  LogLevel,
  Gender,
  Nationality,
  getGenderArray,
  getNationalityArray
};

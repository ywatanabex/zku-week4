import React from "react"
import * as ethers from "ethers"

export default function Watch() {
  const provider = new ethers.providers.JsonRpcProvider("http://localhost:8545/");
  const greeters_address = "0xe7f1725e7734ce288f8367e1bb143e90bb3f0512";
  const ABI = require('../artifacts/contracts/Greeters.sol/Greeters.json');
  const contract = new ethers.Contract(greeters_address, ABI['abi'], provider);
  console.log("contract", contract);

  const [logs, setLogs] = React.useState("not received yet")
  React.useEffect(() => {
    contract.on('NewGreeting', (greeting) => { setLogs(ethers.utils.parseBytes32String((greeting))) });
  }, [])  

  return (
    <>
      <h1>Greeting Message</h1>
      <h2>
        {logs}
      </h2>
    </>
  );
}
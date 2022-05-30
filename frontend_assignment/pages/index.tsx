import detectEthereumProvider from "@metamask/detect-provider"
import { Strategy, ZkIdentity } from "@zk-kit/identity"
import { generateMerkleProof, Semaphore } from "@zk-kit/protocols"
import { providers } from "ethers"
import Head from "next/head"
import React from "react"
import * as Yup from "yup";
import { Formik, Form, Field, ErrorMessage } from "formik";
import styles from "../styles/Home.module.css"

export default function Home() {
    const [logs, setLogs] = React.useState("Connect your wallet and greet!")

    const validationSchema = Yup.object({
        name: Yup.string().required(),
        email: Yup.string().email().required(),
    });

    const initialValues = {
        name: "",
        email: "",
    };

    const renderError = (message: string) => <p style={{color: "red"}}>{message}</p>;

    async function greet(name: string, email: string) {
        console.log("greet method called")
        setLogs("Creating your Semaphore identity...")
       
        const provider = (await detectEthereumProvider()) as any

        await provider.request({ method: "eth_requestAccounts" })

        const ethersProvider = new providers.Web3Provider(provider)
        const signer = ethersProvider.getSigner()
        const message = await signer.signMessage("Sign this message to create your identity!")

        const identity = new ZkIdentity(Strategy.MESSAGE, message)
        const identityCommitment = identity.genIdentityCommitment()
        const identityCommitments = await (await fetch("./identityCommitments.json")).json()

        const merkleProof = generateMerkleProof(20, BigInt(0), identityCommitments, identityCommitment)

        setLogs("Creating your Semaphore proof...")

        const greeting = `${name} (${email})`   
        console.log("Greeting Message: ", greeting)
        console.log("Greeting Message up to 25 Characters: ", greeting.slice(0, 25))

        const witness = Semaphore.genWitness(
            identity.getTrapdoor(),
            identity.getNullifier(),
            merkleProof,
            merkleProof.root,
            greeting.slice(0, 25),  // must be less than 32 bytes
        )

        const { proof, publicSignals } = await Semaphore.genProof(witness, "./semaphore.wasm", "./semaphore_final.zkey")
        const solidityProof = Semaphore.packToSolidityProof(proof)

        const response = await fetch("/api/greet", {
            method: "POST",
            body: JSON.stringify({
                greeting,
                nullifierHash: publicSignals.nullifierHash,
                solidityProof: solidityProof
            })
        })

        if (response.status === 500) {
            const errorMessage = await response.text()

            setLogs(errorMessage)
        } else {
            setLogs("Your Name and Email is onchain :)")
        }
    }

    return (
        <div className={styles.container}>
            <Head>
                <title>Greetings</title>
                <meta name="description" content="A simple Next.js/Hardhat privacy application with Semaphore." />
                <link rel="icon" href="/favicon.ico" />
            </Head>

            <main className={styles.main}>
                <h1 className={styles.title}>Send Your Greetings</h1>
                <p className={styles.description}>Your Name and Email address are included in your greetings.</p>

                <Formik 
                    initialValues={initialValues} 
                    validationSchema={validationSchema} 
                    onSubmit={async (values, { resetForm }) => {await greet(values.name, values.email); resetForm()}}
                >
                <Form>
                    <div className="container" style={{width: "60%"}}>
                        <div className="field">
                            <label className="label" htmlFor="name"> Name </label>
                            <Field
                                name="name"
                                type="text"
                                className="input"
                                placeholder="Name"
                            />                    
                            <ErrorMessage name="name" render={renderError} />
                        </div>
                        <div className="field">
                            <label className="label" htmlFor="email"> Email address </label>
                            <Field
                                name="email"
                                type="text"
                                className="input"
                                placeholder="Email address"
                            />
                            <ErrorMessage name="email" render={renderError} />
                        </div>
                    </div>  
                    <p></p>             
                    <div className={styles.logs}>{logs}</div>  
                                         
                    <button type="submit" className={styles.button}> Greet </button>
                </Form>
                </Formik>

            </main>
        </div>
    )
}

import { Connection, DataSizeFilter, PublicKey } from "@solana/web3.js";
import idl from '../idl.json'

export type EndpointTypes = 'mainnet' | 'devnet' | 'localnet'

export const programID = new PublicKey(idl.metadata.address);

export class VoterAccount {

    data:Uint8Array;
    owner: PublicKey;
    preference: PublicKey;
    constructor(data:Uint8Array) {
        this.data = data

        this.owner = new PublicKey(this.getOwner())
        this.preference = new PublicKey(this.getPreference())
    }

    getOwner() {
        return this.data.slice(8, 8+32);
    }

    getPreference() {
        return this.data.slice(8+32, 8+32+32);
    }

    public static async getVoterAccount(connection, pk:PublicKey) {
        let pda = PublicKey.findProgramAddressSync(
            [Buffer.from("vote"), pk.toBytes()],
            programID
        )
        let info_raw = await connection.getAccountInfo(pda[0])
        return new VoterAccount(info_raw)
    }
}

export class CandidateList {

    data:Uint8Array;
    size:number;
    constructor(data:Uint8Array) {
        this.data = data

        this.size = this.getSize().valueOf();
    }

    getSize() {
        return Number(Buffer.from(this.data).readUInt16LE(8))
    }

    public static async getList(connection) {
        let pda = PublicKey.findProgramAddressSync(
            [Buffer.from("list")],
            programID
        )
        let info_raw = await connection.getAccountInfo(pda[0])
        return new CandidateList(info_raw)
    }
}

export class CandidateAccount {

    data:Uint8Array;
    piece:PublicKey;
    votes:number;
    index:number;
    constructor(data:Uint8Array) {
        this.data = data

        this.piece = this.getPiece()
        this.votes = this.getVotes().valueOf()
        this.index = this.getIndex().valueOf()
    }

    getPiece() {
        return new PublicKey(this.data.slice(8, 8+32))
    }

    getVotes() {
        return Number(Buffer.from(this.data).readUInt32LE(8+32))
    }

    getIndex() {
        return Number(Buffer.from(this.data).readUInt16LE(8+32+8))
    }

    public static async getCandidateAt(connection, index) {
        // TODO check!
        let indexSeed = Buffer.from("")
        indexSeed.writeInt16BE(index)
        let pda = PublicKey.findProgramAddressSync(
            [Buffer.from("candidate"), indexSeed],
            programID
        )
        let info_raw = await connection.getAccountInfo(pda[0])
        return new CandidateAccount(info_raw)
    }

    private static getCandidatePdaList(size) {
        let pdaList=[]
        for (let i=0; i<size; i++){
            let indexSeed = Buffer.from("")
            indexSeed.writeInt16BE(i)
            let pda = PublicKey.findProgramAddressSync(
                [Buffer.from("candidate"), indexSeed],
                programID
            )
            pdaList.push(pda)
        }
        return pdaList
    }

    private static async getAllCandidates(connection:Connection) {
        let list = await CandidateList.getList(connection)

        let candidateList = this.getCandidatePdaList(list.size)

        return connection.getMultipleAccountsInfo(candidateList)
    }

    public static async getCandidateObjectList(connection:Connection) {
        let accounts:CandidateAccount[] = []
        let accounts_info = await this.getAllCandidates(connection)
        accounts_info.forEach(element => {
            accounts.push(new CandidateAccount(element.data))
        });

        return accounts.sort((a:CandidateAccount, b:CandidateAccount) => {return Number(a.votes > b.votes)})
    }
}

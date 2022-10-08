import { PublicKey } from "@solana/web3.js";

export type EndpointTypes = 'mainnet' | 'devnet' | 'localnet'

export class VoterAccount {

    data:Uint8Array;
    owner: PublicKey;
    preference: PublicKey;
    constructor(data:Uint8Array) {
        this.data = data

        this.owner = new PublicKey(this.getOwner())
        this.preference = new PublicKey(this.getPreference())
    }

    getData() {
        return this.data;
    }

    getOwner() {
        return this.data.slice(8, 8+32);
    }

    getPreference() {
        return this.data.slice(8+32, 8+32+32);
    }
}

export class CandidateList {

    data:Uint8Array;
    size:number;
    constructor(data:Uint8Array) {
        this.data = data

        this.size = this.getSize().valueOf();
    }

    getData() {
        return this.data;
    }

    getSize() {
        return Number(Buffer.from(this.data).readUInt16LE(8))
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

    getData() {
        return this.data;
    }

    getPiece() {
        return new PublicKey(this.data.slice(8, 8+32))
    }

    getVotes() {
        return Number(Buffer.from(this.data).readBigUInt64LE(8+32))
    }

    getIndex() {
        return Number(Buffer.from(this.data).readUInt16LE(8+32+8))
    }
}

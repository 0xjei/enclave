import {
  loadFixture,
  mine,
  time,
} from "@nomicfoundation/hardhat-network-helpers";
import { LeanIMT } from "@zk-kit/lean-imt";
import { expect } from "chai";
import { ZeroHash } from "ethers";
import { ethers } from "hardhat";
import { poseidon2 } from "poseidon-lite";

import { deployCiphernodeRegistryOwnableFixture } from "../fixtures/CiphernodeRegistryOwnable.fixture";
import { naiveRegistryFilterFixture } from "../fixtures/NaiveRegistryFilter.fixture";
import { PoseidonT3Fixture } from "../fixtures/PoseidonT3.fixture";

const abiCoder = ethers.AbiCoder.defaultAbiCoder();
const AddressOne = "0x0000000000000000000000000000000000000001";
const AddressTwo = "0x0000000000000000000000000000000000000002";
const addressThree = "0x0000000000000000000000000000000000000003";

// Hash function used to compute the tree nodes.
const hash = (a: bigint, b: bigint) => poseidon2([a, b]);

describe.only("CiphernodeRegistryOwnable", function () {
  async function setup() {
    const [owner, notTheOwner] = await ethers.getSigners();

    const poseidon = await PoseidonT3Fixture();
    const registry = await deployCiphernodeRegistryOwnableFixture(
      owner.address,
      owner.address,
      await poseidon.getAddress(),
    );
    const filter = await naiveRegistryFilterFixture(
      owner.address,
      await registry.getAddress(),
    );
    await registry.addCiphernode(AddressOne);
    await registry.addCiphernode(AddressTwo);

    return {
      owner,
      notTheOwner,
      registry,
      filter,
      request: {
        e3Id: 1,
        filter: await filter.getAddress(),
        threshold: [2, 2] as [number, number],
      },
    };
  }

  describe("constructor / initialize()", function () {
    it("correctly sets `_owner` and `enclave` ", async function () {
      const poseidonFactory = await ethers.getContractFactory("PoseidonT3");
      const poseidonDeployment = await poseidonFactory.deploy();
      const [deployer] = await ethers.getSigners();
      let ciphernodeRegistryFactory = await ethers.getContractFactory(
        "CiphernodeRegistryOwnable",
        {
          libraries: {
            PoseidonT3: await poseidonDeployment.getAddress(),
          },
        },
      );
      let ciphernodeRegistry = await ciphernodeRegistryFactory.deploy(
        deployer.address,
        AddressTwo,
      );
      expect(await ciphernodeRegistry.owner()).to.equal(deployer.address);
      expect(await ciphernodeRegistry.enclave()).to.equal(AddressTwo);
    });
  });

  describe("requestCommittee()", function () {
    it("reverts if committee has already been requested for given e3Id", async function () {
      const { registry, request } = await loadFixture(setup);
      await registry.requestCommittee(
        request.e3Id,
        request.filter,
        request.threshold,
      );
      await expect(
        registry.requestCommittee(
          request.e3Id,
          request.filter,
          request.threshold,
        ),
      ).to.be.revertedWithCustomError(registry, "CommitteeAlreadyRequested");
    });
    it("stores the registry filter for the given e3Id");
    it("stores the root of the ciphernode registry at the time of the request");
    it("requests a committee from the given filter");
    it("emits a CommitteeRequested event");
    it("reverts if filter.requestCommittee() fails");
    it("returns true if the request is successful");
  });

  describe("publishCommittee()", function () {
    it("reverts if the caller is not the filter for the given e3Id");
    it("stores the public key of the committee");
    it("emits a CommitteePublished event");
  });

  describe("addCiphernode()", function () {
    it("reverts if the caller is not the owner");
    it("adds the ciphernode to the registry");
    it("increments numCiphernodes");
    it("emits a CiphernodeAdded event");
  });

  describe("removeCiphernode()", function () {
    it("reverts if the caller is not the owner");
    it("removes the ciphernode from the registry");
    it("decrements numCiphernodes");
    it("emits a CiphernodeRemoved event");
  });

  describe("setEnclave()", function () {
    it("reverts if the caller is not the owner");
    it("sets the enclave address");
    it("emits an EnclaveSet event");
  });

  describe("committeePublicKey()", function () {
    it("returns the public key of the committee for the given e3Id");
    it("reverts if the committee has not been published");
  });

  describe("isCiphernodeEligible()", function () {
    it("returns true if the ciphernode is in the registry");
    it("returns false if the ciphernode is not in the registry");
  });

  describe("isEnabled()", function () {
    it("returns true if the ciphernode is currently enabled");
    it("returns false if the ciphernode is not currently enabled");
  });

  describe("root()", function () {
    it("returns the root of the ciphernode registry merkle tree");
  });

  describe("rootAt()", function () {
    it(
      "returns the root of the ciphernode registry merkle tree at the given e3Id",
    );
  });
});

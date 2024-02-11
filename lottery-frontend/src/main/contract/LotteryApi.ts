/*
 * Copyright (C) 2022 - 2023 Partisia Blockchain Foundation
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

import { TransactionApi } from "../client/TransactionApi";
import { sign } from "./LotteryGenerated"; // Assuming `LotteryGenerated` contains the generated code for entering the lottery
import { getContractAddress } from "../AppState";

/**
 * API for the lottery contract.
 * This minimal implementation allows for entering the lottery.
 *
 * The implementation uses the TransactionApi to send transactions, and ABI for the contract to be
 * able to build the RPC for the enter transaction.
 */
export class LotteryApi {
  private readonly transactionApi: TransactionApi;

  constructor(transactionApi: TransactionApi) {
    this.transactionApi = transactionApi;
  }

  /**
   * Build and send sign transaction.
   */
  readonly sign = () => {
    const address = getContractAddress();
    if (address === undefined) {
      throw new Error("No address provided");
    }
    // First build the RPC buffer that is the payload of the transaction.
    const rpc = sign();
    // Then send the payload via the transaction API.
    return this.transactionApi.sendTransactionAndWait(address, rpc, 10_000);
  };
}

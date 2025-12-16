/**
 * TypeScript type declarations for protobuf test messages
 */

export interface IUserLoginResponse {
  userId?: string | null;
  userName?: string | null;
  isActive?: boolean | null;
  timestamp?: number | string | null;
  sessionToken?: Uint8Array | null;
}

export class UserLoginResponse implements IUserLoginResponse {
  userId?: string | null;
  userName?: string | null;
  isActive?: boolean | null;
  timestamp?: number | string | null;
  sessionToken?: Uint8Array | null;

  constructor(properties?: IUserLoginResponse);

  static create(properties?: IUserLoginResponse): UserLoginResponse;
  static encode(message: IUserLoginResponse, writer?: any): any;
  static decode(reader: any, length?: number): UserLoginResponse;
  static fromObject(object: { [k: string]: any }): UserLoginResponse;
  static toObject(message: UserLoginResponse, options?: any): { [k: string]: any };
}

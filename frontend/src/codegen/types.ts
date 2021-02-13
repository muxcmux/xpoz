export type Maybe<T> = T | null;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
};


export type Album = {
  __typename?: 'Album';
  id: Scalars['String'];
  title?: Maybe<Scalars['String']>;
  itemsCount: Scalars['Int'];
  photosCount: Scalars['Int'];
  videosCount: Scalars['Int'];
  createdAt: Scalars['String'];
  assets: Array<Asset>;
  keyAssets: Array<Asset>;
};


export type AlbumAssetsArgs = {
  offset: Scalars['Int'];
  limit: Scalars['Int'];
};

export type Asset = {
  __typename?: 'Asset';
  id: Scalars['String'];
  createdAt: Scalars['String'];
  height: Scalars['Int'];
  width: Scalars['Int'];
  latitude: Scalars['Float'];
  longitude: Scalars['Float'];
  duration: Scalars['Float'];
  isVideo: Scalars['Boolean'];
  entity?: Maybe<Entity>;
};

export type Entity = {
  __typename?: 'Entity';
  id: Scalars['Int'];
  name: Scalars['String'];
  parentId?: Maybe<Scalars['Int']>;
  parent?: Maybe<Entity>;
};

export type MutationRoot = {
  __typename?: 'MutationRoot';
  createToken?: Maybe<Token>;
  updateToken?: Maybe<Token>;
  deleteToken?: Maybe<Token>;
};


export type MutationRootCreateTokenArgs = {
  input: TokenInput;
};


export type MutationRootUpdateTokenArgs = {
  id: Scalars['String'];
  input: TokenInput;
};


export type MutationRootDeleteTokenArgs = {
  id: Scalars['String'];
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
  album?: Maybe<Album>;
  myAlbums: Array<Album>;
  me: Token;
  tokens: Array<Token>;
};


export type QueryRootAlbumArgs = {
  id: Scalars['String'];
};


export type QueryRootMyAlbumsArgs = {
  page?: Maybe<Scalars['Int']>;
};

export type Token = {
  __typename?: 'Token';
  id: Scalars['String'];
  name: Scalars['String'];
  sessionBound: Scalars['Boolean'];
  admin: Scalars['Boolean'];
  sessionId?: Maybe<Scalars['String']>;
  createdAt: Scalars['String'];
  whitelistedAlbums?: Maybe<Array<Album>>;
};

export type TokenInput = {
  name: Scalars['String'];
  sessionBound: Scalars['Boolean'];
  admin: Scalars['Boolean'];
  albumIds?: Maybe<Array<Scalars['String']>>;
};

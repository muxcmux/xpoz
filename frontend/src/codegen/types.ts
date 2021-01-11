export type Maybe<T> = T | null;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
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
  id: Scalars['Int'];
  uuid: Scalars['String'];
  title?: Maybe<Scalars['String']>;
  itemsCount: Scalars['Int'];
  photosCount: Scalars['Int'];
  videosCount: Scalars['Int'];
  createdAt: Scalars['String'];
  entity?: Maybe<Entity>;
  assets: Array<Asset>;
  keyAssets: Array<Asset>;
};


export type AlbumAssetsArgs = {
  offset: Scalars['Int'];
  limit: Scalars['Int'];
};

export type Asset = {
  __typename?: 'Asset';
  id: Scalars['Int'];
  uuid: Scalars['String'];
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
  deleteToken: Scalars['Int'];
};


export type MutationRootCreateTokenArgs = {
  name?: Maybe<Scalars['String']>;
  sessionBound: Scalars['Boolean'];
  admin: Scalars['Boolean'];
  whitelist?: Maybe<Scalars['String']>;
};


export type MutationRootDeleteTokenArgs = {
  id: Scalars['String'];
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
  entity?: Maybe<Entity>;
  entities: Array<Entity>;
  album?: Maybe<Album>;
  myAlbums: Array<Album>;
  tokens: Array<Token>;
};


export type QueryRootEntityArgs = {
  id: Scalars['Int'];
};


export type QueryRootAlbumArgs = {
  uuid: Scalars['String'];
};


export type QueryRootMyAlbumsArgs = {
  page: Scalars['Int'];
};

export type Token = {
  __typename?: 'Token';
  name: Scalars['String'];
  sessionBound: Scalars['Boolean'];
  admin: Scalars['Boolean'];
  sessionId?: Maybe<Scalars['String']>;
  token: Scalars['String'];
  createdAt: Scalars['String'];
};

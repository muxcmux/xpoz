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
  page: Scalars['Int'];
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
  entity?: Maybe<Entity>;
};

export type Entity = {
  __typename?: 'Entity';
  id: Scalars['Int'];
  name: Scalars['String'];
  parentId?: Maybe<Scalars['Int']>;
  parent?: Maybe<Entity>;
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
  /** Gets a single entity by it's id */
  entity?: Maybe<Entity>;
  /** Returns all available entities in the photos app */
  entities: Array<Entity>;
  /** Get an album by it's uuid */
  album?: Maybe<Album>;
  /** "My Albums" which have been xpozed, keeping the original Photos sorting */
  myAlbums: Array<Album>;
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

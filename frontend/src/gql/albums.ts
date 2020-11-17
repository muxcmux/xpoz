import gql from "./gql";

export const getAllAlbums = gql`
  query getAllAlbums {
    myAlbums {
      id
      uuid
      title
    }
  }
`;

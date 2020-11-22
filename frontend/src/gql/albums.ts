export const getMyAlbums = `
  query {
    myAlbums {
      uuid
      title
      photosCount
      videosCount
      createdAt
      keyAssets {
        uuid
      }
    }
  }
`;

export const getAlbum = `
  query($uuid: String!) {
    album(uuid: $uuid) {
      uuid
      title
      photosCount
      videosCount
      assets {
        uuid
      }
    }
  }
`;

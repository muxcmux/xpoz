export const getMyAlbums = `
  query myAlbums($page: Int!) {
    myAlbums(page: $page) {
      id
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
  query album($uuid: String!, $page: Int!) {
    album(uuid: $uuid) {
      uuid
      title
      photosCount
      videosCount
      assets(page: $page) {
        id
        uuid
      }
    }
  }
`;

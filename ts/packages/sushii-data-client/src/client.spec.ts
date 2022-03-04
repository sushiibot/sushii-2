import ApiClient from "./client";

describe("Client", () => {
  let client: ApiClient;

  beforeAll(async () => {
    client = new ApiClient("http://localhost:3000");
  });

  it("should be defined", () => {
    expect(client).toBeDefined();
  });

  describe("guild-configs", () => {
    it("should fetch default guild config if not found", () => {
      return expect(client.getGuildConfig("1234")).resolves.toEqual(
        expect.objectContaining({
          id: "1234",
        })
      );
    });
  });

  describe("users", () => {
    it("should fetch default user if not found", () => {
      return expect(client.getUser("1234")).resolves.toEqual(
        expect.objectContaining({
          id: "1234",
        })
      );
    });
  });
});

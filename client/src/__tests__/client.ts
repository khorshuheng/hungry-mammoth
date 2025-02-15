import { UserApi } from "../api";

describe("API integration test", () => {
  const userApiClient = new UserApi();
  test("should create users", async () => {
    const response = await userApiClient.newUser({
      email: "user1@domain.com",
      password: "password1",
    });
    expect(response.status).toBe(201);
  });
  test("should throw error when email is duplicated", async () => {
    await expect(
      userApiClient.newUser({
        email: "user1@domain.com",
        password: "password2",
      }),
    ).rejects.toMatchObject({
      response: {
        status: 409,
      },
    });
  });
  test("should list users", async () => {
    const response = await userApiClient.listUsers();
    expect(response.status).toBe(200);
    expect(response.data.users).toEqual([
      {
        email: "user1@domain.com",
        id: 1,
      },
    ]);
  });
});

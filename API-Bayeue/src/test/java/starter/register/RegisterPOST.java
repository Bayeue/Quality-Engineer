package starter.register;

import net.serenitybdd.rest.SerenityRest;
import net.thucydides.core.annotations.Step;
import org.json.simple.JSONObject;

import static net.serenitybdd.rest.SerenityRest.restAssuredThat;
import static org.hamcrest.Matchers.equalTo;

public class RegisterPOST {

    protected String url = "https://virtserver.swaggerhub.com/gozza/Payment-Point/1.0.0-beta/api/users/";

    @Step("I set POST API Register endpoint")
    public String setPostApiRegisterEndpoint() {
        return url + "register";

    }
    @Step("I send Post Register HTTP request")
    public void sendPostRegisterHttpRequest() {
        JSONObject requestBody = new JSONObject();
        requestBody.put("name", "Wulan");
        requestBody.put("email", "wlndrisri4@gmail.com");
        requestBody.put("password", "Palemban9");
        requestBody.put("phonenumber", "+628123456789");
        requestBody.put("image", "example.jpg");

        SerenityRest.given().header("Content-Type", "application/json").body(requestBody.toJSONString()).post(setPostApiRegisterEndpoint());

    }
    @Step("I receive valid Register HTTP response code 200")
    public void receiveValidRegisterHttpResponseCode200() {
        restAssuredThat(response -> response.statusCode(200));

    }
    @Step("I receive valid data for detail user")
    public void receiveValidDataForDetailUser() {
        restAssuredThat(response -> response.body("'message'", equalTo("success create account!")));


    }
}

package starter.login;

import net.serenitybdd.rest.SerenityRest;
import net.thucydides.core.annotations.Step;
import org.json.simple.JSONObject;

import static net.serenitybdd.rest.SerenityRest.restAssuredThat;
import static org.hamcrest.Matchers.equalTo;

public class LoginPOST {

    protected String url = "https://virtserver.swaggerhub.com/gozza/Payment-Point/1.0.0-beta/api/users/";

    @Step("I set POST API Login endpoint")
    public String setPostApiLoginEndpoint() {
        return url + "login";

    }
    @Step("I send Post Login HTTP request")
    public void sendPostLoginHttpRequest() {
        JSONObject requestBody = new JSONObject();
        requestBody.put("email", "wlndrisri4@gmail.com");
        requestBody.put("password", "Palemban9");

        SerenityRest.given().header("Content-Type", "application/json").body(requestBody.toJSONString()).post(setPostApiLoginEndpoint());

    }
    @Step("I receive valid Login HTTP response code 200")
    public void receiveValidLoginHttpResponseCode200() {
        restAssuredThat(response -> response.statusCode(200));

    }
    @Step("I receive valid data to login")
    public void receiveValidDataToLogin() {
        restAssuredThat(response -> response.body("'message'", equalTo("success login!")));


    }





}

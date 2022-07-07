package starter.stepdefinition;

import io.cucumber.java.en.And;
import io.cucumber.java.en.Given;
import io.cucumber.java.en.Then;
import io.cucumber.java.en.When;
import net.thucydides.core.annotations.Steps;
import starter.login.LoginPOST;

public class LoginSteps {
    @Steps
    LoginPOST post;

    @Given("I set Post Api Login endpoint")
    public void setPostApiLoginEndpoint() {
        post.setPostApiLoginEndpoint();

    }
    @When("I send Post Login http request")
    public void sendPostLoginHttpRequest() {
        post.sendPostLoginHttpRequest();

    }
    @Then("I receive valid Login http response code 200")
    public void receiveValidLoginHttpResponseCode200() {
        post.receiveValidLoginHttpResponseCode200();

    }
    @And("I receive valid data to login")
    public void receiveValidDataToLogin() {
        post.receiveValidDataToLogin();
    }





}

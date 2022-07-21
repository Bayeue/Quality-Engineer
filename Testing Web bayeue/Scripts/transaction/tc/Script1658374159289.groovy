import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://bayeue.vercel.app/login')

WebUI.setText(findTestObject('Object Repository/Page_Login Page - Bayeue/input_Email_input-13 (2)'), 'admin@gmail.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Login Page - Bayeue/input_Password_input-16 (2)'), 'RigbBhfdqOBGNlJIWM1ClA==')

WebUI.click(findTestObject('Object Repository/Page_Login Page - Bayeue/button_Login (2)'))

WebUI.click(findTestObject('Object Repository/Page_Dashboard Page - Bayeue/div_Transactions (1)'))

WebUI.click(findTestObject('Object Repository/Page_Transactions Page - Bayeue/button_PAID_v-icon notranslate text-center _916bd3 (1)'))

WebUI.click(findTestObject('Object Repository/Page_Invoice Page - Bayeue/a_Check (1)'))

WebUI.delay(3)

WebUI.back()

WebUI.back()

WebUI.click(findTestObject('Object Repository/Page_Transactions Page - Bayeue/button_PAID_v-icon notranslate text-center _916bd3 (1)'))

WebUI.click(findTestObject('Object Repository/Page_Invoice Page - Bayeue/a_Check_1'))

WebUI.delay(3)

WebUI.back()

WebUI.back()

WebUI.closeBrowser(FailureHandling.STOP_ON_FAILURE)


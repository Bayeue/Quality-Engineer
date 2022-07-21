<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>delete category product</name>
   <tag></tag>
   <elementGuidId>8c2b140c-8c9a-4527-a685-7d68e3fbadae</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6MiwicGhvbmUiOiIrNjI4OTAwMDAwMDAwMDAiLCJzdGF0dXMiOnRydWV9.kZeecXCmggdLl6hdgmO-g20hPt07mrxGOzkVYZLo9vY</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6MiwicGhvbmUiOiIrNjI4OTAwMDAwMDAwMDAiLCJzdGF0dXMiOnRydWV9.kZeecXCmggdLl6hdgmO-g20hPt07mrxGOzkVYZLo9vY</value>
      <webElementGuid>9836b3a0-ac4c-408a-9533-1e0df3d76873</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>https://bayeue.thisham.my.id/api/admin/category/delete/18</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseurl</defaultValue>
      <description></description>
      <id>b4decf4e-224f-4346-9311-0fa95e1b4205</id>
      <masked>false</masked>
      <name>baseurl</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

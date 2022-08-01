<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>codeCreation_Sandbox</name>
   <tag></tag>
   <elementGuidId>00f3c423-e13a-46ae-9125-df07207d9743</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;provider_type_id\&quot;: 5,\n  \&quot;name\&quot;: \&quot;aug0208\&quot;,\n  \&quot;description\&quot;: \&quot;Testing\&quot;,\n  \&quot;access_type_id\&quot;: 6,\n  \&quot;user_name\&quot;: \&quot;nandha0709\&quot;,\n  \&quot;vcs_org_ids\&quot;: [\n    \&quot;Testingqa2022\&quot;\n  ],\n  \&quot;password\&quot;: \&quot;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\&quot;,\n  \&quot;sync_user_repos\&quot;: true\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>2182b8de-9e77-4058-84e9-814d8e932561</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>path</name>
      <type>Main</type>
      <value>/api/admin/provider</value>
      <webElementGuid>6fdff297-6521-4e11-a5ce-75a58855afe1</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>461d9dfe5cc5367307ebb055941291e127c2b9c58401d1bfd89251913613412f</value>
      <webElementGuid>4a0d3c45-7592-447d-9d96-c465d483823b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-workspace-id</name>
      <type>Main</type>
      <value>62e77b440bcb6a68a900cd70</value>
      <webElementGuid>d7981bb6-4fcf-46dc-b4fd-aba6b5d68c16</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://sandbox.ozone.one/api/admin/provider</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
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
